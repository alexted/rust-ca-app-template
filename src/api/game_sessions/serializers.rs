use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::data::repositories::game_sessions_dto::GameSessionDTO;
use crate::data::repositories::query_stuff::ResultPaging;
use crate::use_cases::game_sessions::models::StartGameRequest;

#[derive(Deserialize, Serialize)]
pub struct StartGameSerializer {
    session_id: Option<i64>,
    user_id: Option<i64>,
    game_id: Option<i64>,
    currency: Option<i64>
}

#[derive(Debug, Serialize)]
pub struct GameSessionSerializer {
    id: i64,
    session_id: Option<i64>,
    user_id: Option<i64>,
    game_id: Option<i64>,
    currency: Option<i64>,
    start_time: NaiveDateTime,
}

impl Into<GameSessionSerializer> for GameSessionDTO {
    fn into(self) -> GameSessionSerializer {
        GameSessionSerializer {
            id: self.id,
            session_id: self.session_id,
            user_id: self.user_id,
            game_id: self.game_id,
            currency: self.currency,
            start_time: self.start_time, // мб стоит задать здесь дефолтное значение
        }
    }
}

impl Into<StartGameRequest> for StartGameSerializer {
    fn into(self) -> StartGameRequest {
        StartGameRequest {
            session_id: self.session_id,
            user_id: self.user_id,
            game_id: self.game_id,
            currency: self.currency,
            // start_time: self.start_time,
        }
    }
}

// impl Into<GameSessionDTO> for StartGameSerializer {
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

// impl Into<StartGameSerializer> for GameSessionDTO {
//     fn into(self) -> StartGameSerializer {
//         StartGameSerializer {
//             session_id: self.session_id,
//             user_id: self.user_id,
//             game_id: self.game_id,
//             currency: self.currency,
//         }
//     }
// }

impl Into<ResultPaging<GameSessionSerializer>> for ResultPaging<GameSessionDTO> {
    fn into(self) -> ResultPaging<GameSessionSerializer> {
        ResultPaging {
            total: self.total,
            items: self.items.into_iter().map(|game| game.into()).collect(),
        }
    }
}