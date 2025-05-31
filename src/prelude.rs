// Re-export commonly used Bevy types
pub use bevy::prelude::*;

// Re-export commonly used external crate types
pub use brtk::prelude::*;

// Common game components available at the root of the prelude
pub use crate::core::components::{Position, Description};
pub use crate::core::states::GameState;

// Specific areas nested within their own module for self-documenting use
pub mod core {
    pub use crate::core::components::*;
    pub use crate::core::resources::*;
    pub use crate::core::events::*;
    pub use crate::core::states::*;
}

pub mod gameplay {
    pub mod player {
        pub use crate::gameplay::player::components::*;
        pub use crate::gameplay::player::systems::*;
        pub use crate::gameplay::player::events::*;
    }
    
    pub mod enemies {
        pub use crate::gameplay::enemies::components::*;
        pub use crate::gameplay::enemies::systems::*;
        pub use crate::gameplay::enemies::ai::*;
    }
    
    pub mod turns {
        pub use crate::gameplay::turns::components::*;
        pub use crate::gameplay::turns::systems::*;
        pub use crate::gameplay::turns::resources::*;
    }
    
    pub mod world {
        pub use crate::gameplay::world::components::*;
        pub use crate::gameplay::world::systems::*;
        pub use crate::gameplay::world::generation::*;
    }
}

pub mod rendering {
    pub use crate::rendering::components::*;
    pub use crate::rendering::systems::*;
    pub use crate::rendering::resources::*;
    pub use crate::rendering::screens::*;
}

pub mod ui {
    pub use crate::ui::components::*;
    pub use crate::ui::systems::*;
}

pub mod assets {
    pub use crate::assets::entities::*;
}

pub mod utils {
    pub use crate::utils::*;
}
