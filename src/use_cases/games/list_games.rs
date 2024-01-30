// use std::sync::Arc;
//
// use async_trait::async_trait;
//
// use crate::data::error::CommonError;
// use crate::data::repositories::game_sessions_dto::{GameSessionDTO};
// use crate::data::repositories::query_stuff::ResultPaging;
// use crate::data::repositories::base::{GameQueryParams, GameSessionRepository};
// use crate::use_cases::base_use_case::StartGameUseCase;
//
// #[derive(Clone)]
// pub struct StartGameUseCaseImpl {
//     pub repository: Arc<dyn GameSessionRepository>,
// }
//
// impl StartGameUseCaseImpl {
//     pub fn new(repository: Arc<dyn GameSessionRepository>) -> Self {
//         StartGameUseCaseImpl {
//             repository,
//         }
//     }
// }
//
// #[async_trait]
// impl StartGameUseCase for StartGameUseCaseImpl {
//     async fn create(&self, game: GameSessionDTO) -> Result<GameDTO, CommonError> {
//         let mut cloned = game.clone();
//         self.repository
//             .create(&mut cloned)
//             .await
//             .map_err(|e| -> CommonError { e.into() })
//     }
//
//     async fn list(&self, params: GameQueryParams) -> Result<ResultPaging<GameDTO>, CommonError> {
//         self.repository
//             .list(params)
//             .await
//             .map_err(|e| -> CommonError { e.into() })
//     }
//
//     async fn get(&self, game_id: i32) -> Result<GameDTO, CommonError> {
//         self.repository
//             .get(game_id)
//             .await
//             .map_err(|e| -> CommonError { e.into() })
//     }
//
//     async fn delete(&self, game_id: i32) -> Result<(), CommonError> {
//         self.repository
//             .delete(game_id)
//             .await
//             .map_err(|e| -> CommonError { e.into() })
//     }
// }
