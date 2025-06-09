use bevy::prelude::*;

use crate::{
    core::{
        components::tag::PlayerTag,
        events::{CombatEvent, DamageDealtEvent, EntityDeathEvent, GameEndReason, GameEnded},
        resources::CurrentMap,
    },
    debug_combat,
};

/// System that handles combat events for logging and effects
pub fn handle_combat_events(mut combat_events: EventReader<CombatEvent>) {
    for event in combat_events.read() {
        match event {
            CombatEvent::AttackHit { attacker, target, damage } => {
                debug_combat!("Entity {:?} hit entity {:?} for {} damage", attacker, target, damage);
            }
            CombatEvent::AttackMissed { attacker, target } => {
                debug_combat!("Entity {:?} missed entity {:?}", attacker, target);
            }
            CombatEvent::CriticalHit { attacker, target, damage } => {
                debug_combat!(
                    "Entity {:?} critically hit entity {:?} for {} damage!",
                    attacker,
                    target,
                    damage
                );
            }
        }
    }
}

/// System that handles damage dealt events for effects and logging
pub fn handle_damage_events(mut damage_events: EventReader<DamageDealtEvent>) {
    for event in damage_events.read() {
        debug_combat!(
            "Damage dealt: {} damage from {:?} to {:?} at position {:?}",
            event.damage,
            event.attacker,
            event.target,
            event.position
        );

        // TODO: Add visual effects, screen shake, etc. here
        // TODO: Add damage numbers floating text
        // TODO: Add blood splatter effects
    }
}

/// System that handles entity death events
pub fn handle_entity_death(
    mut commands: Commands,
    mut death_events: EventReader<EntityDeathEvent>,
    mut game_end_events: EventWriter<GameEnded>,
    mut map: ResMut<CurrentMap>,
    player_query: Query<Entity, With<PlayerTag>>,
) {
    for death_event in death_events.read() {
        debug_combat!("Entity {:?} died at position {:?}", death_event.entity, death_event.position);

        // Remove the entity from the map
        map.remove_actor(death_event.entity);

        // Check if the dead entity was the player
        if player_query.contains(death_event.entity) {
            debug_combat!("Player died! Game over.");
            game_end_events.write(GameEnded { reason: GameEndReason::PlayerDeath });
        }

        // Despawn the entity
        commands.entity(death_event.entity).despawn();
    }
}
