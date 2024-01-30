// use serde::{Deserialize, Serialize};
//
// use crate::data::repositories::games_dto::{GameDTO, CreateGameDTO};
// use crate::data::repositories::query_stuff::ResultPaging;
//
// #[derive(Deserialize, Serialize)]
// pub struct CreateGameSerializer {
//     pub title: String,
//     pub description: String,
// }
//
// #[derive(Debug, Serialize)]
// pub struct GameSerializer {
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
// impl Into<CreateGameDTO> for CreateGameSerializer {
//     fn into(self) -> CreateGameDTO {
//         CreateGameDTO {
//             provider_id: self.provider_id,
//             code: self.code,
//             name: self.name,
//             image: self.image,
//             is_mobile: self.is_mobile,
//             is_desktop: self.is_desktop,
//             is_tablet: self.is_tablet,
//             is_demo: self.is_demo,
//             is_embedded: self.is_embedded,
//             bonus: self.bonus,
//             free_spins: self.free_spins,
//             is_active: self.is_active,
//             created_at: self.created_at,
//             updated_at: self.updated_at,
//         }
//     }
// }
//
// impl Into<CreateGameSerializer> for CreateGameDTO {
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