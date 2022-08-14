use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error in setup: {0}")]
    SetupEnvVar(#[from] std::env::VarError),
    #[error("Error in setup: {0}")]
    SetupConfig(#[from] config::ConfigError),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("generic error {0}")]
    Dynamic(#[from] Box<dyn std::error::Error>),
}
