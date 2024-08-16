use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::api::register as regiser_api;

mod api;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let pool = db::init_db();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(web::scope("/api/v1").configure(regiser_api))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
