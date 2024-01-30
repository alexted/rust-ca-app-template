use std::sync::Arc;

use async_trait::async_trait;

use crate::data::error::CommonError;
use crate::data::repositories::game_sessions_dto::CreateGameSessionDTO;
use crate::data::repositories::game_sessions_repo::GameSessionsRepository;
use crate::use_cases::game_sessions::models::{StartGameRequest, StartGameResponse};

#[derive(Clone)]
pub struct StartGameUseCaseImpl {
    pub repository: Arc<dyn GameSessionsRepository>,
}

impl StartGameUseCaseImpl {
    pub fn new(repository: Arc<dyn GameSessionsRepository>) -> Self {
        StartGameUseCaseImpl { repository }
    }
}

#[async_trait]
pub trait StartGameUseCase: Sync + Send {
    async fn execute(&self, req_obj: StartGameRequest) -> Result<StartGameResponse, CommonError>;
}

#[async_trait]
impl StartGameUseCase for StartGameUseCaseImpl {
    async fn execute(&self, req_obj: StartGameRequest) -> Result<StartGameResponse, CommonError> {
        // let mut req_obj_cloned = req_obj.clone();

        let result = self.repository
            .create(&CreateGameSessionDTO::from(req_obj))
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        Ok(result.into())
    }
}
