// use actix_threadpool::run;
// use async_trait::async_trait;
// use diesel::prelude::*;
//
// use crate::data::repositories::base::{BaseDieselRepository, GameQueryParams};
// use crate::data::repositories::games_dto::{CreateGameDTO, GameDTO};
// use crate::data::repositories::query_stuff::{QueryParams, RepositoryResult, ResultPaging};
// use crate::infrastructure::databases::postgres::models::games::Game;
// use crate::infrastructure::error::DieselRepositoryError;
//
// #[async_trait]
// pub trait GamesRepository: Send + Sync {
//     async fn create(&self, new_game: &CreateGameDTO) -> RepositoryResult<GameDTO>;
//     async fn list(&self, params: GameQueryParams) -> RepositoryResult<ResultPaging<GameDTO>>;
//     async fn get(&self, game_id: i32) -> RepositoryResult<GameDTO>;
//     async fn delete(&self, game_id: i32) -> RepositoryResult<()>;
// }
//
// #[async_trait]
// impl GamesRepository for BaseDieselRepository {
//     async fn create(&self, new_game: &CreateGameDTO) -> RepositoryResult<GameDTO> {
//         use crate::infrastructure::databases::postgres::schema::games::dsl::games;
//         let new_game_session: Game = Game::from(new_game.clone());
//         let mut conn = self.pool.get().unwrap();
//         let result: Game = run(move || diesel::insert_into(games).values(new_game_session)
//             .get_result(&mut conn))
//             .await
//             .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
//         Ok(result.into())
//     }
//
//     async fn list(&self, params: GameQueryParams) -> RepositoryResult<ResultPaging<GameDTO>> {
//         use crate::infrastructure::databases::postgres::schema::games::dsl::games;
//         let pool = self.pool.clone();
//         let builder = games.limit(params.limit()).offset(params.offset());
//         let result = run(move || {
//             let mut conn = pool.get().unwrap();
//             builder.load::<Game>(&mut conn)
//         })
//             .await
//             .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
//         Ok(ResultPaging {
//             total: 0,
//             items: result.into_iter().map(|v| v.into()).collect(),
//         })
//     }
//
//     async fn get(&self, game_id: i32) -> RepositoryResult<GameDTO> {
//         use crate::infrastructure::databases::postgres::schema::games::dsl::{games, id};
//         let mut conn = self.pool.get().unwrap();
//         run(move || games.filter(id.eq(game_id)).first::<Game>(&mut conn))
//             .await
//             .map_err(|v| DieselRepositoryError::from(v).into_inner())
//             .map(|v| -> GameDTO { v.into() })
//     }
//
//     async fn delete(&self, game_id: i32) -> RepositoryResult<()> {
//         use crate::infrastructure::databases::postgres::schema::games::dsl::{games, id};
//         let mut conn = self.pool.get().unwrap();
//         run(move || diesel::delete(games).filter(id.eq(game_id))
//             .execute(&mut conn))
//             .await
//             .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
//         Ok(())
//     }
// }