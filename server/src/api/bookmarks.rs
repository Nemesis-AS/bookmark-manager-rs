use actix_web::{
    error,
    web::{self, Query},
    HttpResponse, Responder,
};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, TextExpressionMethods};
use uuid::Uuid;

use crate::db::models::{self, Bookmark};
use crate::db::types::{DbError, DbPool};

use super::{JsonResponse, PaginationQuery};

#[derive(Debug, Clone, serde::Deserialize)]
struct TagFilterList {
    tags: String,
}

async fn get_all_bookmarks(
    pool: web::Data<DbPool>,
    query: Query<PaginationQuery>,
) -> actix_web::Result<impl Responder> {
    let query_params: PaginationQuery = query.into_inner();
    let limit: i64 = query_params.limit.unwrap_or(25);
    let offset: i64 = (query_params.page.unwrap_or(1) - 1) * limit;

    let bookmarks = web::block(move || -> Result<Vec<models::Bookmark>, DbError> {
        use crate::db::schema::bookmarks::dsl::*;

        let mut conn = pool.get()?;

        let term: String = query_params.term.clone().unwrap_or("".to_string());

        let res: Vec<models::Bookmark> = bookmarks
            .filter(title.like(format!("%{}%", term)))
            .order(created_at.desc())
            .limit(limit)
            .offset(offset)
            .load(&mut conn)?;

        Ok(res)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let res = JsonResponse {
        success: true,
        message: "Bookmarks retrieved successfully".to_string(),
        data: Some(serde_json::to_value(bookmarks).unwrap()),
        page: Some(query_params.page.unwrap_or(1)),
        limit: Some(limit),
    };
    Ok(HttpResponse::Ok().json(res))
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

    let res = JsonResponse {
        success: true,
        message: "Bookmarks retrieved successfully".to_string(),
        data: Some(serde_json::to_value(bookmark).unwrap()),
        page: None,
        limit: None,
    };
    Ok(HttpResponse::Ok().json(res))
}

async fn filter_bookmarks_by_tag(
    pool: web::Data<DbPool>,
    query: web::Query<TagFilterList>,
) -> actix_web::Result<impl Responder> {
    let tag_str: String = query.into_inner().tags;
    let filter_tags: Vec<Uuid> = tag_str
        .split(",")
        .map(|str| Uuid::parse_str(&str).unwrap())
        .collect::<Vec<Uuid>>();

    let bookmarks = web::block(move || -> Result<Vec<Bookmark>, DbError> {
        use crate::db::schema::bookmarks::dsl::*;

        let mut conn = pool.get()?;

        let res: Vec<Bookmark> = bookmarks.load::<models::Bookmark>(&mut conn)?;

        let out: Vec<Bookmark> = res
            .into_iter()
            .filter(|b| {
                let bookmark_tags: Vec<Uuid> = b
                    .tags
                    .split(",")
                    .map(|str| Uuid::parse_str(&str).unwrap())
                    .collect::<Vec<Uuid>>();

                filter_tags.iter().all(|tag| bookmark_tags.contains(tag))
            })
            .collect::<Vec<Bookmark>>();

        Ok(out)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let res: JsonResponse = JsonResponse {
        success: true,
        message: "Filtered Bookmark Successfully".to_string(),
        data: Some(serde_json::to_value(bookmarks).unwrap()),
        page: None,
        limit: None,
    };
    Ok(HttpResponse::Ok().json(res))
}

async fn create_bookmark(
    pool: web::Data<DbPool>,
    data: web::Json<models::NewBookmark>,
) -> actix_web::Result<impl Responder> {
    let bookmark = web::block(move || -> Result<models::Bookmark, DbError> {
        use crate::db::schema::bookmarks::dsl::*;

        let mut conn = pool.get()?;

        let new_bookmark = models::Bookmark {
            id: Uuid::new_v4().to_string(),
            url: data.url.clone(),
            title: data.title.clone(),
            description: data.description.clone(),
            tags: data.tags.clone(),
            created_at: None,
        };

        diesel::insert_into(bookmarks)
            .values(&new_bookmark)
            .execute(&mut conn)?;

        Ok(new_bookmark)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let res = JsonResponse {
        success: true,
        message: "Bookmark created successfully".to_string(),
        data: Some(serde_json::to_value(bookmark).unwrap()),
        page: None,
        limit: None,
    };
    Ok(HttpResponse::Ok().json(res))
}

async fn update_bookmark(
    pool: web::Data<DbPool>,
    data: web::Json<models::Bookmark>,
) -> actix_web::Result<impl Responder> {
    let bookmark = web::block(move || -> Result<models::Bookmark, DbError> {
        use crate::db::schema::bookmarks::dsl::*;

        let mut conn = pool.get()?;

        let updated_bookmark = data.into_inner();

        diesel::update(bookmarks)
            .filter(id.eq(updated_bookmark.id.clone()))
            .set(&updated_bookmark)
            .execute(&mut conn)?;

        Ok(updated_bookmark)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let res = JsonResponse {
        success: true,
        message: "Bookmark updated successfully".to_string(),
        data: Some(serde_json::to_value(bookmark).unwrap()),
        page: None,
        limit: None,
    };

    Ok(HttpResponse::Ok().json(res))
}

async fn delete_bookmark(
    pool: web::Data<DbPool>,
    uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    web::block(move || -> Result<(), DbError> {
        use crate::db::schema::bookmarks::dsl::*;

        let mut conn = pool.get()?;

        diesel::delete(bookmarks.filter(id.eq(uid.into_inner().to_string()))).execute(&mut conn)?;

        Ok(())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let res: JsonResponse = JsonResponse {
        success: true,
        message: "Bookmark deleted successfully".to_string(),
        data: None,
        page: None,
        limit: None,
    };
    Ok(HttpResponse::Ok().json(res))
}

pub fn register(config: &mut actix_web::web::ServiceConfig) {
    config.service(
        web::resource("")
            .route(web::get().to(get_all_bookmarks))
            .route(web::post().to(create_bookmark))
            .route(web::put().to(update_bookmark)),
    );
    config.service(web::resource("/bytag").route(web::get().to(filter_bookmarks_by_tag)));
    config.service(
        web::resource("/{uid}")
            .route(web::get().to(get_bookmark_by_id))
            .route(web::delete().to(delete_bookmark)),
    );
}
