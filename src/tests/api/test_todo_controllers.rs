#[cfg(test)]
mod test_game_controllers {
    use actix_web::test;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations};
    use game_service::create_app::create_app;
    use game_service::domain::constants::POSTGRESQL_DB_URI;
    use game_service::domain::models::game::Game;
    use game_service::domain::repositories::repository::ResultPaging;
    use game_service::infrastructure::databases::postgresql::db_pool;
    use serde_json;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    #[actix_web::test]
    async fn test() {
        env::set_var("RUST_BACKTRACE", "1");
        env::set_var("RUST_LOG", "debug");
        env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();

        let docker = clients::Cli::default();
        let postgres_node = docker.run(postgres::Postgres::default());
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres", postgres_node.get_host_port_ipv4(5432)
        );

        env::set_var(POSTGRESQL_DB_URI, connection_string);

        let pool = Arc::new(db_pool());
        pool.get().unwrap().run_pending_migrations(MIGRATIONS).unwrap();

        let app = test::init_service(create_app()).await;
        let request_body = json!({
            "title": "test game",
            "description": "Test description"
        });

        // Creation test
        let resp = test::TestRequest::post().uri(&format!("/games")).set_json(&request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let game: Game = test::read_body_json(resp).await;
        assert_eq!(game.title, "test game");
        assert_eq!(game.description, "Test description");

        // Get all test
        let resp = test::TestRequest::get().uri(&format!("/games/{}", game.id)).send_request(&app).await;
        assert!(resp.status().is_success());
        let retrieved_game: Game = test::read_body_json(resp).await;
        assert_eq!(game.id, retrieved_game.id);
        assert_eq!(game.title, retrieved_game.title);

        // Creation test
        let resp = test::TestRequest::post().uri(&format!("/games")).set_json(&request_body).send_request(&app).await;
        assert!(resp.status().is_success());

        // Get all test
        let req = test::TestRequest::get().uri("/games").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let games: ResultPaging<Game> = test::read_body_json(resp).await;
        assert_eq!(games.items.len(), 2);

        // Delete test
        let resp = test::TestRequest::delete().uri(&format!("/games/{}", game.id)).send_request(&app).await;
        assert!(resp.status().is_success());

        // Get all test
        let req = test::TestRequest::get().uri("/games").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let games: ResultPaging<Game> = test::read_body_json(resp).await;
        assert_eq!(games.items.len(), 1);
    }
}