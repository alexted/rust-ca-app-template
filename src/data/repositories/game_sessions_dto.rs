use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::use_cases::game_sessions::models::{StartGameRequest, StartGameResponse};

#[derive(Clone, Deserialize)]
pub struct GameSessionDTO {
    pub id: i64,
    pub session_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
    pub currency: Option<i64>,
    pub start_time: NaiveDateTime,
}

impl Into<StartGameResponse> for GameSessionDTO {
    fn into(self) -> StartGameResponse {
        StartGameResponse {
            id: self.id,
            session_id: self.session_id,
            user_id: self.user_id,
            game_id: self.game_id,
            currency: self.currency,
            start_time: self.start_time,
        }
    }
}

#[derive(Clone)]
pub struct CreateGameSessionDTO {
    pub session_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
    pub currency: Option<i64>,
    // pub start_time: Option<NaiveDateTime>,
}
// Для конвертации StartGameRequest в CreateGameSessionDTO
impl From<StartGameRequest> for CreateGameSessionDTO {
    fn from(t: StartGameRequest) -> Self {
        CreateGameSessionDTO {
            session_id: t.session_id,
            user_id: t.user_id,
            game_id: t.game_id,
            currency: t.currency,
            // start_time: t.start_time,
        }
    }
}
