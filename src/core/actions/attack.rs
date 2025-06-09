use bevy::prelude::*;
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

#[derive(Clone, Debug)]
pub struct AttackAction {
    entity: Entity,
    target_position: Position,
}

impl AttackAction {
    pub fn new(entity: Entity, target_position: Position) -> Self { Self { entity, target_position } }

    fn calculate_damage(&self, attacker_stats: &Stats, defender_stats: &Stats) -> i32 {
        let base_damage = 10; // Base weapon damage
        let strength_bonus = attacker_stats.melee_damage_bonus();
        let defense_reduction = defender_stats.damage_reduction();

        let raw_damage = base_damage + strength_bonus;
        let final_damage = (raw_damage - defense_reduction).max(1); // Minimum 1 damage

        // Apply critical hit chance
        let mut rng = Rng::new();
        let crit_chance = attacker_stats.critical_chance();
        let is_critical = rng.f32() * 100.0 <= crit_chance;

        if is_critical {
            debug_combat!("Critical hit!");
            ((final_damage as f32) * 1.5).round() as i32
        } else {
            final_damage
        }
    }

    fn calculate_accuracy(&self, attacker_stats: &Stats, defender_stats: &Stats) -> bool {
        let base_accuracy = 85.0; // 85% base hit chance
        let accuracy_bonus = attacker_stats.accuracy_bonus() as f32 * 2.0;
        let evasion_penalty = defender_stats.evasion_bonus() as f32 * 2.0;

        let final_accuracy = (base_accuracy + accuracy_bonus - evasion_penalty).clamp(5.0, 95.0);

        let mut rng = Rng::new();
        rng.f32() * 100.0 <= final_accuracy
    }
}

impl GameAction for AttackAction {
    fn action_type(&self) -> ActionType { ActionType::Attack(self.target_position) }

    fn execute(&mut self, world: &mut World) -> Result<u64, GameError> {
        debug_combat!("Entity {:?} attacks position {:?}", self.entity, self.target_position);

        // Get the current map to find the target
        let target_entity = {
            let map = world.resource::<CurrentMap>();
            map.get_actor(self.target_position)
        };

        let target_entity = match target_entity {
            Some(entity) => entity,
            None => {
                debug_combat!("No target at position {:?}", self.target_position);
                return Ok(self.duration());
            }
        };

        // Get attacker stats
        let attacker_stats = world.get::<Stats>(self.entity).cloned().unwrap_or_default();
        let defender_stats = world.get::<Stats>(target_entity).cloned().unwrap_or_default();

        // Calculate hit chance
        if !self.calculate_accuracy(&attacker_stats, &defender_stats) {
            debug_combat!("Attack missed!");
            world.send_event(CombatEvent::AttackMissed { attacker: self.entity, target: target_entity });
            return Ok(self.duration());
        }

        // Calculate damage
        let damage = self.calculate_damage(&attacker_stats, &defender_stats);
        debug_combat!("Attack hits for {} damage!", damage);

        // Apply damage to target and collect result
        let (actual_damage, target_died) = {
            let mut target_health = world.get_mut::<Health>(target_entity);
            if let Some(ref mut health) = target_health {
                let actual_damage = health.take_damage(damage);
                let target_died = health.is_dead();
                (actual_damage, target_died)
            } else {
                debug_combat!("Target has no health component!");
                return Err(GameError::InvalidWeapon("Target cannot take damage".to_string()));
            }
        };

        // Send events after releasing the mutable borrow
        world.send_event(DamageDealtEvent {
            attacker: self.entity,
            target: target_entity,
            damage: actual_damage,
            position: self.target_position,
        });

        world.send_event(CombatEvent::AttackHit {
            attacker: self.entity,
            target: target_entity,
            damage: actual_damage,
        });

        // Check if target died
        if target_died {
            debug_combat!("Target died!");
            world.send_event(EntityDeathEvent {
                entity: target_entity,
                position: self.target_position,
                killer: Some(self.entity),
            });
        }

        Ok(self.duration())
    }

    fn duration(&self) -> u64 { self.action_type().get_base_time_to_perform() }
}
