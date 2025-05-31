use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Invalid position")]
    InvalidPosition,
    #[error("Terrain blocked")]
    TerrainBlocked,
    #[error("Entity not found")]
    EntityNotFound,
    #[error("Missing component")]
    MissingComponent,
}
