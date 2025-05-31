use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    core::{
        components::{AITag, Position},
        pathfinding::PathfindingAlgorithm,
    },
    gameplay::{
        enemies::{
            components::{
                AIBehavior, AIBehaviorType, AIState, AIAction, ChasePlayerAction, ChasePlayerScorer,
                FleeFromPlayerAction, FleeFromPlayerScorer, WanderAction, WanderScorer,
            },
            pathfinding::AIPathfinding,
        },
        turns::components::TurnActor,
    },
};

/// Enhanced AI entity bundle with pathfinding capabilities
#[derive(Bundle)]
pub struct EnhancedAIBundle {
    pub ai_tag: AITag,
    pub position: Position,
    pub turn_actor: TurnActor,
    pub ai_behavior: AIBehavior,
    pub ai_state: AIState,
    pub ai_pathfinding: AIPathfinding,
    pub thinker: Thinker,
}

impl EnhancedAIBundle {
    /// Create a new enhanced AI bundle with specified behavior
    pub fn new(
        position: Position,
        behavior_type: AIBehaviorType,
        detection_range: u8,
        speed: u64,
        pathfinding_algorithm: PathfindingAlgorithm,
    ) -> Self {
        let ai_behavior = match behavior_type {
            AIBehaviorType::Hostile => AIBehavior::hostile(detection_range),
            AIBehaviorType::Passive => AIBehavior::passive(detection_range),
            AIBehaviorType::Neutral => AIBehavior::neutral(detection_range),
        };

        let ai_pathfinding = AIPathfinding::new(pathfinding_algorithm)
            .with_max_iterations(1000)
            .with_diagonal(false);

        let thinker = create_thinker_for_behavior(&behavior_type);

        Self {
            ai_tag: AITag,
            position,
            turn_actor: TurnActor::new(speed),
            ai_behavior,
            ai_state: AIState {
                current_action: AIAction::Idle,
                target_position: None,
                last_action_time: 0.0,
            },
            ai_pathfinding,
            thinker,
        }
    }

    /// Create a hostile AI with enhanced pathfinding
    pub fn hostile(
        position: Position,
        detection_range: u8,
        speed: u64,
    ) -> Self {
        Self::new(
            position,
            AIBehaviorType::Hostile,
            detection_range,
            speed,
            PathfindingAlgorithm::AStar,
        )
    }

    /// Create a passive AI with enhanced pathfinding
    pub fn passive(
        position: Position,
        detection_range: u8,
        speed: u64,
    ) -> Self {
        Self::new(
            position,
            AIBehaviorType::Passive,
            detection_range,
            speed,
            PathfindingAlgorithm::AStar,
        )
    }

    /// Create a neutral AI with enhanced pathfinding
    pub fn neutral(
        position: Position,
        detection_range: u8,
        speed: u64,
    ) -> Self {
        Self::new(
            position,
            AIBehaviorType::Neutral,
            detection_range,
            speed,
            PathfindingAlgorithm::Dijkstra, // Use Dijkstra for wandering behavior
        )
    }

    /// Create an AI with custom pathfinding settings
    pub fn with_pathfinding_config(
        mut self,
        algorithm: PathfindingAlgorithm,
        allow_diagonal: bool,
        max_iterations: usize,
    ) -> Self {
        self.ai_pathfinding = AIPathfinding::new(algorithm)
            .with_diagonal(allow_diagonal)
            .with_max_iterations(max_iterations);
        self
    }
}

/// Create a thinker component based on AI behavior type
fn create_thinker_for_behavior(behavior_type: &AIBehaviorType) -> Thinker {
    match behavior_type {
        AIBehaviorType::Hostile => {
            Thinker::build()
                .picker(FirstToScore { threshold: 0.8 })
                .when(ChasePlayerScorer, ChasePlayerAction)
                .when(WanderScorer, WanderAction)
                .otherwise(WanderAction)
        }
        AIBehaviorType::Passive => {
            Thinker::build()
                .picker(FirstToScore { threshold: 0.6 })
                .when(FleeFromPlayerScorer, FleeFromPlayerAction)
                .when(WanderScorer, WanderAction)
                .otherwise(WanderAction)
        }
        AIBehaviorType::Neutral => {
            Thinker::build()
                .picker(FirstToScore { threshold: 0.4 })
                .when(WanderScorer, WanderAction)
                .otherwise(WanderAction)
        }
    }
}

/// Spawn an enhanced AI entity with pathfinding
pub fn spawn_enhanced_ai_entity(
    commands: &mut Commands,
    position: Position,
    behavior_type: AIBehaviorType,
    detection_range: u8,
    speed: u64,
    name: Option<String>,
) -> Entity {
    let mut entity_commands = commands.spawn(EnhancedAIBundle::new(
        position,
        behavior_type,
        detection_range,
        speed,
        PathfindingAlgorithm::AStar,
    ));

    if let Some(name) = name {
        entity_commands.insert(Name::new(name));
    }

    entity_commands.id()
}

/// Spawn multiple enhanced AI entities
pub fn spawn_enhanced_ai_group(
    commands: &mut Commands,
    positions: &[Position],
    behavior_type: AIBehaviorType,
    detection_range: u8,
    speed: u64,
    name_prefix: Option<&str>,
) -> Vec<Entity> {
    positions
        .iter()
        .enumerate()
        .map(|(i, &position)| {
            let name = name_prefix.map(|prefix| format!("{} {}", prefix, i + 1));
            spawn_enhanced_ai_entity(
                commands,
                position,
                behavior_type,
                detection_range,
                speed,
                name,
            )
        })
        .collect()
}

/// System to upgrade existing AI entities with pathfinding capabilities
pub fn upgrade_ai_entities_with_pathfinding(
    mut commands: Commands,
    ai_query: Query<(Entity, &AIBehavior), (With<AITag>, Without<AIPathfinding>)>,
) {
    for (entity, ai_behavior) in ai_query.iter() {
        let algorithm = match ai_behavior.behavior_type {
            AIBehaviorType::Hostile | AIBehaviorType::Passive => PathfindingAlgorithm::AStar,
            AIBehaviorType::Neutral => PathfindingAlgorithm::Dijkstra,
        };

        let pathfinding = AIPathfinding::new(algorithm)
            .with_max_iterations(1000)
            .with_diagonal(false);

        commands.entity(entity).insert(pathfinding);
        
        info!("Upgraded AI entity {:?} with pathfinding capabilities", entity);
    }
}

/// Configuration for AI pathfinding behavior
#[derive(Resource, Clone)]
pub struct AIPathfindingConfig {
    pub default_algorithm: PathfindingAlgorithm,
    pub max_iterations: usize,
    pub allow_diagonal: bool,
    pub recalculation_threshold: u32,
    pub enable_path_caching: bool,
}

impl Default for AIPathfindingConfig {
    fn default() -> Self {
        Self {
            default_algorithm: PathfindingAlgorithm::AStar,
            max_iterations: 1000,
            allow_diagonal: false,
            recalculation_threshold: 5,
            enable_path_caching: true,
        }
    }
}

/// System to apply global pathfinding configuration changes
pub fn apply_pathfinding_config(
    config: Res<AIPathfindingConfig>,
    mut ai_query: Query<&mut AIPathfinding>,
) {
    if config.is_changed() {
        for mut pathfinding in ai_query.iter_mut() {
            pathfinding.pathfinder = pathfinding.pathfinder
                .with_max_iterations(config.max_iterations)
                .with_diagonal(config.allow_diagonal);
            pathfinding.recalculate_threshold = config.recalculation_threshold;
        }
        info!("Applied pathfinding configuration changes to all AI entities");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_ai_bundle_creation() {
        let position = Position::new(5, 5);
        let bundle = EnhancedAIBundle::hostile(position, 6, 100);
        
        assert_eq!(bundle.position, position);
        assert_eq!(bundle.ai_behavior.behavior_type, AIBehaviorType::Hostile);
        assert_eq!(bundle.ai_behavior.detection_range, 6);
        assert_eq!(bundle.turn_actor.speed, 100);
    }

    #[test]
    fn test_pathfinding_config() {
        let config = AIPathfindingConfig::default();
        
        assert_eq!(config.default_algorithm, PathfindingAlgorithm::AStar);
        assert_eq!(config.max_iterations, 1000);
        assert!(!config.allow_diagonal);
        assert_eq!(config.recalculation_threshold, 5);
        assert!(config.enable_path_caching);
    }
}
