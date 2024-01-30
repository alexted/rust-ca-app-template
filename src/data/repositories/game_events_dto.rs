use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct GameEventDTO {
    pub id: i64,
    pub provider_id: Option<i64>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub is_mobile: Option<bool>,
    pub is_desktop: Option<bool>,
    pub is_tablet: Option<bool>,
    pub is_demo: Option<bool>,
    pub is_embedded: Option<bool>,
    pub bonus: Option<i32>,
    pub free_spins: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone)]
pub struct CreateGameEventDTO {
    pub session_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
    pub currency: Option<i64>,
    pub start_time: NaiveDateTime,
}
