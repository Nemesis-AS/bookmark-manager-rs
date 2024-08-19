use actix_web::web;

use bookmarks::register as register_bookmarks;

pub mod bookmarks;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

pub fn register(config: &mut web::ServiceConfig) {
    config.service(web::scope("/bookmarks").configure(register_bookmarks));
}
