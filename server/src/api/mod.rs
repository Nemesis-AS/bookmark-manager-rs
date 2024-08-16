use actix_web::web;

use bookmarks::register as register_bookmarks;

pub mod bookmarks;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(web::scope("/bookmarks").configure(register_bookmarks));
}
