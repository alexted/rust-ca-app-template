use actix_web::{App, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::middleware::Logger;

use crate::api::game_sessions::handlers::start_game;
// use crate::api::game_events::handlers::create_game_event;
// use crate::api::games::{create_game, delete_game, get_game, list_games};
use crate::infrastructure::app::container::Container;

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Config=(),
        InitError=(),
        Error=Error,
    >,
> {
    let container = Container::new();
    let start_game_use_case = container.start_game_use_case.clone();

    App::new()
        .app_data(web::Data::from(start_game_use_case.clone()))
        .wrap(Logger::default())
        .service(
            web::scope("/game_sessions")
                .route("", web::post().to(start_game))
        )
    // .service(
    //     web::scope("/game_events")
    //         .route("", web::post().to(create_game_event))
    // )
    // .service(
    //     web::scope("/games")
    //         .route("", web::post().to(create_game))
    //         .route("", web::get().to(list_games))
    //         .route("/{id}", web::get().to(get_game))
    //         .route("/{id}", web::delete().to(delete_game))
    // )
}
