mod views;

use actix_web::web;

use views::register as register_views;

pub fn register(config: &mut web::ServiceConfig) -> () {
    config.service(web::scope("").configure(register_views));
}