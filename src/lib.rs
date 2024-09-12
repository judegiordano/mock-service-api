pub mod aws;
pub mod controllers;
pub mod env;
pub mod errors;
pub mod models;
pub mod types;

pub mod logger {
    use tracing_subscriber::FmtSubscriber;

    use crate::{env::Env, errors::AppError};

    pub fn init() -> Result<(), AppError> {
        let Env { log_level, .. } = Env::load()?;
        let subscriber: FmtSubscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
        tracing::subscriber::set_global_default(subscriber)
            .map_err(AppError::internal_server_error)?;
        Ok(())
    }
}
