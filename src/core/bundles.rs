use bevy::prelude::*;
use echos_assets::entities::AIBehaviorType;
use leafwing_input_manager::prelude::InputMap;

use crate::{
    core::components::*,
    gameplay::{
        enemies::components::{AIBehavior, AIComponent, AIState},
        player::{actions::PlayerAction, components::AwaitingInput},
    },
};

#[derive(Bundle, Default)]
pub struct ActorBundle {
    pub mob: Mob,
    // pub fov: ViewShed,
    pub position: Position,
    pub description: Description,
}

impl ActorBundle {
    /// Create a new actor bundle with a name and position
    pub fn new(name: impl ToString, position: Position) -> Self {
        Self {
            mob: Mob,
            position,
            // name: Name::new(name),
            // fov: ViewShed::new(radius),
            description: Description::new(name.to_string()),
        }
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: PlayerTag,
    pub awaiting_input: AwaitingInput,
    pub input_map: InputMap<PlayerAction>,
    pub position: Position,
    pub description: Description,
    // pub actor: ActorBundle,
}

impl PlayerBundle {
    pub fn new(name: impl ToString, position: Position) -> Self {
        Self {
            position,
            player: PlayerTag,
            awaiting_input: AwaitingInput,
            description: Description::new(name),
            input_map: Self::default_input_map(),
        }
    }

    pub fn default_input_map() -> InputMap<PlayerAction> {
        InputMap::new([
            /////////////////////////////
            // Movement
            /////////////////////////////
            // ArrowKeys
            (PlayerAction::North, KeyCode::ArrowUp),
            (PlayerAction::South, KeyCode::ArrowDown),
            (PlayerAction::West, KeyCode::ArrowLeft),
            (PlayerAction::East, KeyCode::ArrowRight),
            // WSAD
            (PlayerAction::North, KeyCode::KeyW),
            (PlayerAction::South, KeyCode::KeyS),
            (PlayerAction::West, KeyCode::KeyA),
            (PlayerAction::East, KeyCode::KeyD),
            // Diagonals
            (PlayerAction::NorthWest, KeyCode::KeyY),
            (PlayerAction::NorthEast, KeyCode::KeyU),
            (PlayerAction::SouthWest, KeyCode::KeyB),
            (PlayerAction::SouthEast, KeyCode::KeyN),
            /////////////////////////////
            // Actions
            /////////////////////////////
            // Wait
            (PlayerAction::Wait, KeyCode::Period),
            (PlayerAction::Wait, KeyCode::Numpad5),
        ])
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub ai_tag: AITag,
    pub ai_state: AIState,
    pub ai_behavior: AIBehavior,

    pub ai: AIComponent,
    pub actor: ActorBundle,
}

impl EnemyBundle {
    pub fn new(name: impl ToString, position: Position, behavior_type: AIBehaviorType) -> Self {
        let ai_state = AIState::default();
        let ai = AIComponent::new(behavior_type);
        let actor = ActorBundle::new(name, position);
        let ai_behavior = Self::create_ai_behavior_for_type(behavior_type);

        Self { ai_tag: AITag, ai_state, ai_behavior, ai, actor }
    }

    fn create_ai_behavior_for_type(behavior_type: AIBehaviorType) -> AIBehavior {
        match behavior_type {
            AIBehaviorType::Hostile => AIBehavior::hostile(6),
            AIBehaviorType::Passive => AIBehavior::passive(5),
            AIBehaviorType::Neutral => AIBehavior::neutral(3),
        }
    }
}
