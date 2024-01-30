use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct GameDTO {
    pub id: i32,
    pub provider_id: i32,
    pub code: String,
    pub name: String,
    pub image: String,
    pub is_mobile: bool,
    pub is_desktop: bool,
    pub is_tablet: bool,
    pub is_demo: bool,
    pub is_embedded: bool,
    pub bonus: i32,
    pub free_spins: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone)]
pub struct CreateGameDTO {
    pub provider_id: i32,
    pub code: String,
    pub name: String,
    pub image: String,
    pub is_mobile: bool,
    pub is_desktop: bool,
    pub is_tablet: bool,
    pub is_demo: bool,
    pub is_embedded: bool,
    pub bonus: i32,
    pub free_spins: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}


#[derive(Clone, Deserialize)]
pub struct ProviderDTO {
    pub id: i32,
    pub name: String,
    pub aggregator: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone)]
pub struct CreateProviderDTO {
    pub id: i32,
    pub name: String,
    pub aggregator: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}