// Re-export commonly used Bevy types
pub use bevy::prelude::*;

// Re-export commonly used external crate types
pub use brtk::prelude::*;

// Common game components available at the root of the prelude
pub use crate::core::{
    components::{Description, Position},
    states::GameState,
};

// Specific areas nested within their own module for self-documenting use
pub mod core {
    pub use crate::core::{components::*, events::*, resources::*, states::*};
}

pub mod gameplay {
    pub mod player {
        pub use crate::gameplay::player::{components::*, events::*, systems::*};
    }

    pub mod enemies {
        pub use crate::gameplay::enemies::{ai::*, components::*, systems::*};
    }

    pub mod turns {
        pub use crate::gameplay::turns::{components::*, systems::*};
    }

    pub mod world {
        pub use crate::gameplay::world::{components::*, generation::*, systems::*};
    }
}

pub mod rendering {
    pub use crate::rendering::{components::*, resources::*, screens::*, systems::*};
}

pub mod ui {
    pub use crate::ui::{components::*, systems::*};
}

pub mod assets {
    pub use echos_assets::entities::*;
}

pub mod utils {
    pub use crate::utils::*;
}
