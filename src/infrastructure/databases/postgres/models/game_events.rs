// use bigdecimal::BigDecimal;
// use chrono::NaiveDateTime;
// use diesel;
// use diesel::prelude::*;
//
// use crate::data::repositories::game_events_dto::{CreateGameEventDTO, GameEventDTO};
// use crate::infrastructure::databases::postgres::schema::game_events;
//
// #[derive(Queryable, Insertable, Selectable, Debug)]
// #[diesel(table_name = game_events)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct GameEvent {
//     pub id: i64,
//     pub session_id: Option<i64>,
//     pub provider_id: Option<i64>,
//     pub game_id: Option<i64>,
//     pub user_id: Option<i64>,
//     pub currency: Option<i64>,
//     pub spin_id: Option<i64>,
//     pub balance_before: Option<BigDecimal>,
//     pub balance_after: Option<BigDecimal>,
//     pub bet: Option<BigDecimal>,
//     pub win: Option<BigDecimal>,
//     pub profit: Option<BigDecimal>,
//     pub transaction_create_time: NaiveDateTime,
//     pub transaction_finished_time: NaiveDateTime,
// }
//
//
// // Factory method for creating a new GamesDiesel from a Game
// impl From<GameEventDTO> for GameEvent {
//     fn from(t: GameEventDTO) -> Self {
//         GameEvent {
//             id: t.id,
//             session_id: t.session_id,
//             provider_id: t.provider_id,
//             game_id: t.game_id,
//             user_id: t.user_id,
//             currency: t.currency,
//             spin_id: t.spin_id,
//             balance_before: t.balance_before,
//             balance_after: t.balance_after,
//             bet: t.bet,
//             win: t.win,
//             profit: t.profit,
//             transaction_create_time: t.transaction_create_time,
//             transaction_finished_time: t.transaction_finished_time,
//         }
//     }
// }
//
// // Factory method for creating a new Game from a GamesDiesel
// impl Into<GameEventDTO> for GameEvent {
//     fn into(self) -> GameEventDTO {
//         GameEventDTO {
//             id: self.id,
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
// #[derive(Insertable)]
// #[diesel(table_name = game_events)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct CreateGameEvent {
//     pub session_id: Option<i64>,
//     pub provider_id: Option<i64>,
//     pub game_id: Option<i64>,
//     pub user_id: Option<i64>,
//     pub currency: Option<i64>,
//     pub spin_id: Option<i64>,
//     pub balance_before: Option<BigDecimal>,
//     pub balance_after: Option<BigDecimal>,
//     pub bet: Option<BigDecimal>,
//     pub win: Option<BigDecimal>,
//     pub profit: Option<BigDecimal>,
//     pub transaction_create_time: NaiveDateTime,
//     pub transaction_finished_time: NaiveDateTime,
// }
//
//
// impl From<CreateGameEventDTO> for CreateGameEvent {
//     fn from(t: CreateGameEventDTO) -> Self {
//         CreateGameEvent {
//             session_id: t.session_id,
//             provider_id: t.provider_id,
//             game_id: t.game_id,
//             user_id: t.user_id,
//             currency: t.currency,
//             spin_id: t.spin_id,
//             balance_before: t.balance_before,
//             balance_after: t.balance_after,
//             bet: t.bet,
//             win: t.win,
//             profit: t.profit,
//             transaction_create_time: t.transaction_create_time,
//             transaction_finished_time: t.transaction_finished_time,
//         }
//     }
// }
//
// impl Into<GameEventDTO> for CreateGameEvent {
//     fn into(self) -> GameEventDTO {
//         GameEventDTO {
//             id: self.id,
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