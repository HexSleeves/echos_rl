#[macro_export]
macro_rules! spawn_with_fallback {
    ($entity_defs:expr, $assets:expr, $def_spawn:expr, $fallback_spawn:expr, $entity_type:literal) => {
        if let (Some(entity_definitions), Some(assets)) = ($entity_defs, $assets) {
            match $def_spawn {
                Ok(id) => {
                    info!("{} spawned from definition", $entity_type);
                    id
                }
                Err(e) => {
                    warn!("Failed to spawn {} from definition: {}. Using fallback.", $entity_type, e);
                    $fallback_spawn
                }
            }
        } else {
            info!("Entity definitions not available, using hardcoded {} spawning", $entity_type);
            $fallback_spawn
        }
    };
}
