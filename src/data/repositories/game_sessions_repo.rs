use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::data::repositories::base::BaseDieselRepository;
use crate::data::repositories::game_sessions_dto::{CreateGameSessionDTO, GameSessionDTO};
use crate::data::repositories::query_stuff::RepositoryResult;
use crate::infrastructure::databases::postgres::models::game_sessions::{CreateGameSession, GameSession};
use crate::infrastructure::error::DieselRepositoryError;

#[async_trait]
pub trait GameSessionsRepository: Send + Sync {
    async fn create(&self, new_game: &CreateGameSessionDTO) -> RepositoryResult<GameSessionDTO>;
}


#[async_trait]
impl GameSessionsRepository for BaseDieselRepository {
    async fn create(&self, new_game: &CreateGameSessionDTO) -> RepositoryResult<GameSessionDTO> {
        use crate::infrastructure::databases::postgres::schema::game_sessions::dsl::game_sessions;
        let new_game_session: CreateGameSession = CreateGameSession::from(new_game.clone());
        let mut connection = self.pool.get().unwrap();
        let result: GameSession = run(move || diesel::insert_into(game_sessions).values(new_game_session)
            .get_result(&mut connection))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }
}