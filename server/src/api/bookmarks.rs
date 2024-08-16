use actix_web::{error, web, HttpResponse, Responder};
use diesel::{r2d2, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SqliteConnection};
// use diesel::prelude::*;
use uuid::Uuid;

use crate::db::models;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;

async fn get_all_bookmarks(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let bookmarks = web::block(move || -> Result<Vec<models::Bookmark>, DbError> {
        use crate::db::schema::bookmarks::dsl::*;

        let mut conn = pool.get()?;

        let res: Vec<models::Bookmark> = bookmarks.load(&mut conn)?;

        Ok(res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(bookmarks))
}

async fn get_bookmark_by_id(
    pool: web::Data<DbPool>,
    uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let bookmark = web::block(move || -> Result<Option<models::Bookmark>, DbError> {
        use crate::db::schema::bookmarks::dsl::*;

        let mut conn = pool.get()?;

        let uuid: String = uid.into_inner().to_string();

        let res: Option<models::Bookmark> = bookmarks
            .filter(id.eq(uuid))
            .first::<models::Bookmark>(&mut conn)
            .optional()?;

        Ok(res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(bookmark))
}

async fn filter_bookmarks_by_tag() -> impl Responder {
    HttpResponse::Ok().body("[WIP] Filter bookmarks by tag")
}

async fn create_bookmark(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let bookmark = web::block(move || -> Result<models::Bookmark, DbError> {
        use crate::db::schema::bookmarks::dsl::*;

        let mut conn = pool.get()?;

        // let res: Vec<models::Bookmark> = bookmarks.load(&mut conn)?;
        let new_bookmark = models::Bookmark {
            id: Uuid::new_v4().to_string(),
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            description: "An example bookmark".to_string(),
        };

        diesel::insert_into(bookmarks)
            .values(&new_bookmark)
            .execute(&mut conn)?;

        Ok(new_bookmark)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    println!("{:?}", bookmark);

    Ok(HttpResponse::Ok().body("Created Bookmark Successfully!"))
}

async fn update_bookmark() -> impl Responder {
    HttpResponse::Ok().body("[WIP] Update bookmark")
}

async fn delete_bookmark() -> impl Responder {
    HttpResponse::Ok().body("[WIP] Delete bookmark")
}

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config.service(
        web::resource("")
            .route(web::get().to(get_all_bookmarks))
            .route(web::post().to(create_bookmark)),
    );
    config.service(
        web::resource("/{uid}")
            .route(web::get().to(get_bookmark_by_id))
            .route(web::put().to(update_bookmark))
            .route(web::delete().to(delete_bookmark)),
    );
    config.service(web::resource("/tag/{tag}").route(web::get().to(filter_bookmarks_by_tag)));
}
