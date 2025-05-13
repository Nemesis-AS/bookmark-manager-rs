use actix_files::Files;
use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) -> () {
    config
        .service(Files::new("/", "./assets").index_file("index.html"));
}
