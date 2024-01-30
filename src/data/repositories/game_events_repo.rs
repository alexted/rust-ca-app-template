// use actix_threadpool::run;
// use async_trait::async_trait;
// use diesel::prelude::*;
//
// use crate::data::repositories::base::{BaseDieselRepository, GameQueryParams};
// use crate::data::repositories::game_events_dto::{CreateGameEventDTO, GameEventDTO};
// use crate::data::repositories::query_stuff::{QueryParams, RepositoryResult, ResultPaging};
// use crate::infrastructure::databases::postgres::models::game_events::GameEvent;
// use crate::infrastructure::error::DieselRepositoryError;
//
// #[async_trait]
// pub trait GameEventsRepository: Send + Sync {
//     async fn create(&self, new_game: &CreateGameEventDTO) -> RepositoryResult<GameEventDTO>;
// }
//
// #[async_trait]
// impl GameEventsRepository for BaseDieselRepository {
//     async fn create(&self, new_game_event: &CreateGameEventDTO) -> RepositoryResult<GameEventDTO> {
//         use crate::infrastructure::databases::postgres::schema::game_events::dsl::game_events;
//         let new_game_session: GameEvent = GameEvent::from(new_game_event.clone());
//         let mut conn = self.pool.get().unwrap();
//         let result: GameEvent = run(move || diesel::insert_into(game_events).values(new_game_session)
//             .get_result(&mut conn))
//             .await
//             .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
//         Ok(result.into())
//     }
// }