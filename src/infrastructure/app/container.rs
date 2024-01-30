use std::sync::Arc;

use dotenv::dotenv;

use crate::data::repositories::base::BaseDieselRepository;
use crate::data::repositories::game_sessions_repo::GameSessionsRepository;
use crate::infrastructure::app::config::Config;
use crate::infrastructure::databases::postgres::connection::db_pool;
use crate::use_cases::game_sessions::start_session::{StartGameUseCase, StartGameUseCaseImpl};

pub struct Container {
    pub start_game_use_case: Arc<dyn StartGameUseCase>,
    // WIP
    // pub store_game_event_use_case: Arc<dyn StartGameUseCase>,
    // pub sync_games_use_case: Arc<dyn StartGameUseCase>,
}

impl Container {
    pub fn new() -> Self {
        dotenv().ok();

        let config = envy::from_env::<Config>().unwrap();


        let game_sessions_repository: Arc<dyn GameSessionsRepository> = Arc::new(
            BaseDieselRepository::new(Arc::new(db_pool(config.yugabyte_dsn)))
        );

        // Use cases
        let start_game_use_case = Arc::new(
            StartGameUseCaseImpl { repository: game_sessions_repository }
        );

        Container { start_game_use_case }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
