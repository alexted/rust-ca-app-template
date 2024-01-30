// use std::sync::Arc;
//
// use async_trait::async_trait;
//
// use crate::data::error::CommonError;
// use crate::data::repositories::game_events_repo::GameEventsRepository;
// use crate::use_cases::game_events::models::{StoreGameEventRequest, StoreGameEventResponse};
//
// #[derive(Clone)]
// pub struct StoreGameEventUseCaseImpl {
//     pub repository: Arc<dyn GameEventsRepository>,
// }
//
// impl StoreGameEventUseCaseImpl {
//     pub fn new(repository: Arc<dyn GameEventsRepository>) -> Self {
//         StoreGameEventUseCaseImpl {
//             repository,
//         }
//     }
// }
//
// #[async_trait]
// pub trait StoreGameEventUseCase: Sync + Send {
//     async fn execute(&self, req_obj: StoreGameEventRequest) -> Result<StoreGameEventResponse, CommonError>;
// }
//
// #[async_trait]
// impl StoreGameEventUseCase for StoreGameEventUseCaseImpl {
//     async fn execute(&self, req_obj: StoreGameEventRequest) -> Result<StoreGameEventResponse, CommonError> {
//         let mut cloned = req_obj.clone();
//         self.repository
//             .create(&mut cloned)
//             .await
//             .map_err(|e| -> CommonError { e.into() })
//     }
// }
