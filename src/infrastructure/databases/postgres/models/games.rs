use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;

use crate::data::repositories::games_dto::{CreateGameDTO, CreateProviderDTO, GameDTO, ProviderDTO};
use crate::infrastructure::databases::postgres::schema::{
    games, providers,
};

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
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

impl From<GameDTO> for Game {
    fn from(t: GameDTO) -> Self {
        Game {
            id: t.id,
            provider_id: t.provider_id,
            code: t.code,
            name: t.name,
            image: t.image,
            is_mobile: t.is_mobile,
            is_desktop: t.is_desktop,
            is_tablet: t.is_tablet,
            is_demo: t.is_demo,
            is_embedded: t.is_embedded,
            bonus: t.bonus,
            free_spins: t.free_spins,
            is_active: t.is_active,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

// Factory method for creating a new Game from a GamesDiesel
impl Into<GameDTO> for Game {
    fn into(self) -> GameDTO {
        GameDTO {
            id: self.id,
            provider_id: self.provider_id,
            code: self.code,
            name: self.name,
            image: self.image,
            is_mobile: self.is_mobile,
            is_desktop: self.is_desktop,
            is_tablet: self.is_tablet,
            is_demo: self.is_demo,
            is_embedded: self.is_embedded,
            bonus: self.bonus,
            free_spins: self.free_spins,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = providers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Provider {
    pub id: i32,
    pub name: String,
    pub aggregator: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}


// Factory method for creating a new ProvidersDiesel from a Provider
impl From<ProviderDTO> for Provider {
    fn from(t: ProviderDTO) -> Self {
        Provider {
            id: t.id,
            name: t.name,
            aggregator: t.aggregator,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

// Factory method for creating a new Provider from a ProvidersDiesel
impl Into<ProviderDTO> for Provider {
    fn into(self) -> ProviderDTO {
        ProviderDTO {
            id: self.id,
            name: self.name,
            aggregator: self.aggregator,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = games)]
pub struct CreateGame {
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

impl From<CreateGameDTO> for CreateGame {
    fn from(t: CreateGameDTO) -> Self {
        CreateGame {
            provider_id: t.provider_id,
            code: t.code,
            name: t.name,
            image: t.image,
            is_mobile: t.is_mobile,
            is_desktop: t.is_desktop,
            is_tablet: t.is_tablet,
            is_demo: t.is_demo,
            is_embedded: t.is_embedded,
            bonus: t.bonus,
            free_spins: t.free_spins,
            is_active: t.is_active,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

impl Into<GameDTO> for CreateGame {
    fn into(self) -> GameDTO {
        GameDTO {
            id: 0,
            provider_id: self.provider_id,
            code: self.code,
            name: self.name,
            image: self.image,
            is_mobile: self.is_mobile,
            is_desktop: self.is_desktop,
            is_tablet: self.is_tablet,
            is_demo: self.is_demo,
            is_embedded: self.is_embedded,
            bonus: self.bonus,
            free_spins: self.free_spins,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = providers)]
pub struct CreateProvider {
    pub name: String,
    pub aggregator: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<CreateProviderDTO> for CreateProvider {
    fn from(t: CreateProviderDTO) -> Self {
        CreateProvider {
            name: t.name,
            aggregator: t.aggregator,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

impl Into<CreateProviderDTO> for CreateProvider {
    fn into(self) -> CreateProviderDTO {
        CreateProviderDTO {
            id: 0,
            name: self.name,
            aggregator: self.aggregator,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}