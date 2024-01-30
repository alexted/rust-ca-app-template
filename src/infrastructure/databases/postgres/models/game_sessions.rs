use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;

use crate::data::repositories::game_sessions_dto::{CreateGameSessionDTO, GameSessionDTO};
use crate::infrastructure::databases::postgres::schema::game_sessions;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = game_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GameSession {
    pub id: i64,
    pub session_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
    pub currency: Option<i64>,
    pub start_time: NaiveDateTime,
}

// Factory method for creating a new GamesSession from a GameSessionDTO
impl From<GameSessionDTO> for GameSession {
    fn from(t: GameSessionDTO) -> Self {
        GameSession {
            id: t.id,
            session_id: t.session_id,
            user_id: t.user_id,
            game_id: t.game_id,
            currency: t.currency,
            start_time: t.start_time,
        }
    }
}

// Factory method for converting a GameSession to a GameSessionDTO
impl Into<GameSessionDTO> for GameSession {
    fn into(self) -> GameSessionDTO {
        GameSessionDTO {
            id: self.id,
            session_id: self.session_id,
            user_id: self.user_id,
            game_id: self.game_id,
            currency: self.currency,
            start_time: self.start_time,
        }
    }
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = game_sessions)]
pub struct CreateGameSession {
    pub session_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
    pub currency: Option<i64>,
    // pub start_time: NaiveDateTime,
}

impl From<CreateGameSessionDTO> for CreateGameSession {
    fn from(t: CreateGameSessionDTO) -> Self {
        CreateGameSession {
            session_id: t.session_id,
            user_id: t.user_id,
            game_id: t.game_id,
            currency: t.currency,
            // start_time: t.start_time,
        }
    }
}

// impl Into<GameSessionDTO> for CreateGameSession {
//     fn into(self) -> GameSessionDTO {
//         GameSessionDTO {
//             id: 0,
//             session_id: self.session_id,
//             user_id: self.user_id,
//             game_id: self.game_id,
//             currency: self.currency,
//             start_time: self.start_time,
//         }
//     }
// }

// impl From<CreateGameSessionDTO> for GameSession {
//     fn from(t: CreateGameSessionDTO) -> Self {
//         GameSession {
//             id: 0,
//             session_id: t.session_id,
//             user_id: t.user_id,
//             game_id: t.game_id,
//             currency: t.currency,
//             start_time: t.start_time,
//         }
//     }
// }