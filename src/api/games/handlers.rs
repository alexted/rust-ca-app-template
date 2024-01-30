// use actix_web::{HttpResponse, Result, web};
//
// use crate::api::serializers::game::{CreateGameSerializer, GameSerializer};
// use crate::data::error::ApiError;
// use crate::data::repositories::base::GameQueryParams;
// use crate::data::repositories::query_stuff::ResultPaging;
// use crate::use_cases::base_use_case::StartGameUseCase;
//
// pub async fn create_game(
//     game_service: web::Data<dyn StartGameUseCase>, post_data: web::Json<CreateGameSerializer>,
// ) -> Result<web::Json<GameSerializer>, ApiError> {
//     let game = game_service.create(post_data.into_inner().into()).await?;
//     Ok(web::Json(game.into()))
// }
//
// pub async fn list_games(
//     game_service: web::Data<dyn StartGameUseCase>, params: web::Query<GameQueryParams>,
// ) -> Result<web::Json<ResultPaging<GameSerializer>>, ApiError> {
//     let selection = game_service.list(params.into_inner()).await?;
//     Ok(web::Json(selection.into()))
// }
//
// pub async fn get_game(
//     game_service: web::Data<dyn StartGameUseCase>, params: web::Path<i32>,
// ) -> Result<web::Json<GameSerializer>, ApiError> {
//     let game = game_service.get(params.into_inner()).await?;
//     Ok(web::Json(game.into()))
// }
//
// pub async fn delete_game(
//     game_service: web::Data<dyn StartGameUseCase>, params: web::Path<i32>,
// ) -> Result<HttpResponse, ApiError> {
//     game_service.delete(params.into_inner()).await?;
//     Ok(HttpResponse::NoContent().finish())
// }