use actix_web::HttpServer;

use game_service::infrastructure::app::fabric::create_app;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || { create_app() })
        .bind(("127.0.0.1", 5000))?;

    server.run().await
}
