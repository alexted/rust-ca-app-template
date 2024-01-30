use actix_web::{Result, web};

use crate::data::error::ApiError;
use crate::use_cases::game_sessions::models::{StartGameRequest, StartGameResponse};
use crate::use_cases::game_sessions::start_session::StartGameUseCase;

pub async fn start_game(
    start_game_usecase: web::Data<dyn StartGameUseCase>,
    post_data: web::Json<StartGameRequest>,
) -> Result<web::Json<StartGameResponse>, ApiError> {

    let game_session = start_game_usecase.execute(
        post_data.into_inner().into()
    ).await?;

    Ok(web::Json(game_session))
}
