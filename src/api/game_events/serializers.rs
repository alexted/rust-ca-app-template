// use serde::{Deserialize, Serialize};
//
// use crate::data::repositories::game_sessions_dto::{GameSessionDTO};
// use crate::data::repositories::query_stuff::ResultPaging;
//
// #[derive(Deserialize, Serialize)]
// pub struct CreateGameEventSerializer {
//     pub title: String,
//     pub description: String,
// }
//
// #[derive(Debug, Serialize)]
// pub struct GameEventSerializer {
//     id: i32,
//     title: String,
//     description: String,
//     completed: bool,
// }
//
// impl Into<GameSerializer> for GameDTO {
//     fn into(self) -> GameSerializer {
//         GameSerializer {
//             id: self.id,
//             title: self.title,
//             description: self.description,
//             completed: false,
//         }
//     }
// }
//
// impl Into<GameSessionDTO> for CreateGameSerializer {
//     fn into(self) -> GameSessionDTO {
//         GameSessionDTO {
//             title: self.title,
//             description: self.description,
//         }
//     }
// }
//
// impl Into<CreateGameSerializer> for GameSessionDTO {
//     fn into(self) -> CreateGameSerializer {
//         CreateGameSerializer {
//             title: self.title,
//             description: self.description,
//         }
//     }
// }
//
// impl Into<ResultPaging<GameSerializer>> for ResultPaging<GameDTO> {
//     fn into(self) -> ResultPaging<GameSerializer> {
//         ResultPaging {
//             total: self.total,
//             items: self.items.into_iter().map(|game| game.into()).collect(),
//         }
//     }
// }