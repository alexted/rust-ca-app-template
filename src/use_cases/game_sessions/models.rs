use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct StartGameRequest {
    pub session_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
    pub currency: Option<i64>,
    // pub start_time: Option<NaiveDateTime>,
}

#[derive(Clone, Serialize)]
pub struct StartGameResponse {
    pub id: i64,
    pub session_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
    pub currency: Option<i64>,
    pub start_time: NaiveDateTime,
}
