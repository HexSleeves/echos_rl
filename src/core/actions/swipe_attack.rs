use bevy::prelude::*;
use brtk::prelude::Direction;
use fastrand::Rng;

use crate::{
    core::{
        components::{Health, Position, Stats},
        events::{CombatEvent, DamageDealtEvent, EntityDeathEvent},
        resources::CurrentMap,
        types::{ActionType, GameAction, GameError},
    },
    debug_combat,
};

/// Defines different swipe attack patterns
#[derive(Clone, Debug)]
pub enum SwipePattern {
    /// Horizontal swipe (left-center-right relative to facing direction)
    Horizontal,
    /// Vertical swipe (up-center-down relative to facing direction)
    Vertical,
    /// Diagonal swipe (diagonal line of 3 positions)
    Diagonal,
    /// Arc swipe (3 positions in an arc)
    Arc,
    /// All adjacent positions (8-directional)
    AllAdjacent,
}

impl SwipePattern {
    /// Get the positions to attack based on the pattern and center position
    pub fn get_target_positions(&self, center: Position, facing_direction: Direction) -> Vec<Position> {
        match self {
            SwipePattern::Horizontal => {
                // Get perpendicular directions to facing direction
                let (left, right) = get_perpendicular_directions(facing_direction);
                vec![center + left.coord(), center + facing_direction.coord(), center + right.coord()]
            }
            SwipePattern::Vertical => {
                // For vertical, we use the facing direction and its opposite
                let opposite = get_opposite_direction(facing_direction);
                vec![
                    center + opposite.coord(),
                    center + facing_direction.coord(),
                    center + get_perpendicular_directions(facing_direction).0.coord(),
                ]
            }
            SwipePattern::Diagonal => {
                // Diagonal line through the facing direction
                let diagonal_dirs = get_diagonal_line(facing_direction);
                diagonal_dirs.iter().map(|dir| center + dir.coord()).collect()
            }
            SwipePattern::Arc => {
                // Arc pattern: facing direction and two adjacent directions
                let adjacent_dirs = get_adjacent_directions(facing_direction);
                let mut positions = vec![center + facing_direction.coord()];
                positions.extend(adjacent_dirs.iter().map(|dir| center + dir.coord()));
                positions
            }
            SwipePattern::AllAdjacent => {
                // All 8 adjacent positions
                vec![
                    center + Direction::NORTH.coord(),
                    center + Direction::NORTH_EAST.coord(),
                    center + Direction::EAST.coord(),
                    center + Direction::SOUTH_EAST.coord(),
                    center + Direction::SOUTH.coord(),
                    center + Direction::SOUTH_WEST.coord(),
                    center + Direction::WEST.coord(),
                    center + Direction::NORTH_WEST.coord(),
                ]
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwipeAttackAction {
    entity: Entity,
    center_position: Position,
    pattern: SwipePattern,
    facing_direction: Direction,
}

impl SwipeAttackAction {
    pub fn new(
        entity: Entity,
        center_position: Position,
        pattern: SwipePattern,
        facing_direction: Direction,
    ) -> Self {
        Self { entity, center_position, pattern, facing_direction }
    }

    fn calculate_damage(&self, attacker_stats: &Stats, defender_stats: &Stats) -> i32 {
        let base_damage = 8; // Slightly less than single attack to balance multi-target
        let strength_bonus = attacker_stats.melee_damage_bonus();
        let defense_reduction = defender_stats.damage_reduction();

        let raw_damage = base_damage + strength_bonus;
        let final_damage = (raw_damage - defense_reduction).max(1);

        // Apply critical hit chance
        let mut rng = Rng::new();
        let crit_chance = attacker_stats.critical_chance();
        let is_critical = rng.f32() * 100.0 <= crit_chance;

        if is_critical {
            debug_combat!("Critical swipe hit!");
            ((final_damage as f32) * 1.5).round() as i32
        } else {
            final_damage
        }
    }

    fn calculate_accuracy(&self, attacker_stats: &Stats, defender_stats: &Stats) -> bool {
        let base_accuracy = 80.0; // Slightly lower than single attack
        let accuracy_bonus = attacker_stats.accuracy_bonus() as f32 * 2.0;
        let evasion_penalty = defender_stats.evasion_bonus() as f32 * 2.0;

        let final_accuracy = (base_accuracy + accuracy_bonus - evasion_penalty).clamp(5.0, 95.0);

        let mut rng = Rng::new();
        rng.f32() * 100.0 <= final_accuracy
    }
}

impl GameAction for SwipeAttackAction {
    fn action_type(&self) -> ActionType {
        ActionType::Attack(self.center_position) // Use center position for action type
    }

    fn execute(&mut self, world: &mut World) -> Result<u64, GameError> {
        debug_combat!("Entity {:?} performs swipe attack with pattern {:?}", self.entity, self.pattern);

        // Get target positions based on pattern
        let target_positions = self.pattern.get_target_positions(self.center_position, self.facing_direction);

        // Get attacker stats once
        let attacker_stats = world.get::<Stats>(self.entity).cloned().ok_or(GameError::MissingComponent {
            entity: self.entity,
            component: std::any::type_name::<Stats>(),
        })?;

        let mut targets_hit = 0;

        // Collect all target entities first to avoid borrowing conflicts
        let target_entities: Vec<(Position, Entity)> = {
            let map = world.resource::<CurrentMap>();
            target_positions
                .iter()
                .filter_map(|&target_pos| map.get_actor(target_pos).map(|entity| (target_pos, entity)))
                .collect()
        };

        // Attack each target entity
        for (target_pos, target_entity) in target_entities {
            {
                // Don't attack self
                if target_entity == self.entity {
                    continue;
                }

                // Get defender stats
                let defender_stats = match world.get::<Stats>(target_entity).cloned() {
                    Some(stats) => stats,
                    None => {
                        debug_combat!("Target at {:?} has no stats component", target_pos);
                        continue;
                    }
                };

                // Calculate hit chance
                if !self.calculate_accuracy(&attacker_stats, &defender_stats) {
                    debug_combat!("Swipe attack missed target at {:?}", target_pos);
                    world.send_event(CombatEvent::AttackMissed {
                        attacker: self.entity,
                        target: target_entity,
                    });
                    continue;
                }

                // Calculate damage
                let damage = self.calculate_damage(&attacker_stats, &defender_stats);
                debug_combat!("Swipe hits target at {:?} for {} damage!", target_pos, damage);

                // Apply damage and check for death
                let (actual_damage, target_died) = {
                    let mut target_health = world.get_mut::<Health>(target_entity);
                    if let Some(ref mut health) = target_health {
                        let actual_damage = health.take_damage(damage);
                        let target_died = health.is_dead();
                        (actual_damage, target_died)
                    } else {
                        debug_combat!("Target at {:?} has no health component!", target_pos);
                        continue;
                    }
                };

                // Send events
                world.send_event(DamageDealtEvent {
                    attacker: self.entity,
                    target: target_entity,
                    damage: actual_damage,
                    position: target_pos,
                });

                world.send_event(CombatEvent::AttackHit {
                    attacker: self.entity,
                    target: target_entity,
                    damage: actual_damage,
                });

                if target_died {
                    debug_combat!("Swipe attack killed target at {:?}!", target_pos);
                    world.send_event(EntityDeathEvent {
                        entity: target_entity,
                        position: target_pos,
                        killer: Some(self.entity),
                    });
                }

                targets_hit += 1;
            }
        }

        debug_combat!("Swipe attack hit {} targets", targets_hit);
        Ok(self.duration())
    }

    fn duration(&self) -> u64 {
        // Swipe attacks take slightly longer than regular attacks
        self.action_type().get_base_time_to_perform() + 200
    }
}

// Helper functions for swipe patterns

fn get_perpendicular_directions(dir: Direction) -> (Direction, Direction) {
    match dir {
        Direction::NORTH | Direction::SOUTH => (Direction::WEST, Direction::EAST),
        Direction::EAST | Direction::WEST => (Direction::NORTH, Direction::SOUTH),
        Direction::NORTH_EAST | Direction::SOUTH_WEST => (Direction::NORTH_WEST, Direction::SOUTH_EAST),
        Direction::NORTH_WEST | Direction::SOUTH_EAST => (Direction::NORTH_EAST, Direction::SOUTH_WEST),
        _ => (Direction::NORTH, Direction::SOUTH), // Default fallback
    }
}

fn get_opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::NORTH => Direction::SOUTH,
        Direction::SOUTH => Direction::NORTH,
        Direction::EAST => Direction::WEST,
        Direction::WEST => Direction::EAST,
        Direction::NORTH_EAST => Direction::SOUTH_WEST,
        Direction::SOUTH_WEST => Direction::NORTH_EAST,
        Direction::NORTH_WEST => Direction::SOUTH_EAST,
        Direction::SOUTH_EAST => Direction::NORTH_WEST,
        _ => Direction::SOUTH, // Default fallback
    }
}

fn get_diagonal_line(dir: Direction) -> Vec<Direction> {
    match dir {
        Direction::NORTH_EAST => vec![Direction::SOUTH_WEST, Direction::NORTH_EAST],
        Direction::SOUTH_EAST => vec![Direction::NORTH_WEST, Direction::SOUTH_EAST],
        Direction::SOUTH_WEST => vec![Direction::NORTH_EAST, Direction::SOUTH_WEST],
        Direction::NORTH_WEST => vec![Direction::SOUTH_EAST, Direction::NORTH_WEST],
        _ => vec![dir], // For cardinal directions, just use the direction itself
    }
}

fn get_adjacent_directions(dir: Direction) -> Vec<Direction> {
    let all_dirs = [
        Direction::NORTH,
        Direction::NORTH_EAST,
        Direction::EAST,
        Direction::SOUTH_EAST,
        Direction::SOUTH,
        Direction::SOUTH_WEST,
        Direction::WEST,
        Direction::NORTH_WEST,
    ];

    // Find the index of the current direction
    if let Some(index) = all_dirs.iter().position(|&d| d == dir) {
        let left_index = if index == 0 { all_dirs.len() - 1 } else { index - 1 };
        let right_index = if index == all_dirs.len() - 1 { 0 } else { index + 1 };

        vec![all_dirs[left_index], all_dirs[right_index]]
    } else {
        // Fallback for unknown directions
        vec![Direction::NORTH, Direction::SOUTH]
    }
}
