// use actix_web::{Result, web};
//
// use crate::api::game_events::serializers::{CreateGameEventSerializer, GameEventSerializer};
// use crate::data::error::ApiError;
// use crate::use_cases::game_events::store_game_event::StoreGameEventUseCase;
//
// pub async fn create_game_event(
//     game_service: web::Data<dyn StoreGameEventUseCase>, post_data: web::Json<CreateGameEventSerializer>,
// ) -> Result<web::Json<GameEventSerializer>, ApiError> {
//     let game = game_service.create(post_data.into_inner().into()).await?;
//     Ok(web::Json(game.into()))
// }