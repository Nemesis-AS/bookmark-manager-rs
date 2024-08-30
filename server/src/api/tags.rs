use actix_web::{
    web::{self, Data, Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db::models::{NewTag, Tag};
use crate::db::types::{DbError, DbPool};

use super::JsonResponse;

use uuid::Uuid;

async fn get_all_tags(pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    let tags = web::block(move || -> Result<Vec<Tag>, DbError> {
        use crate::db::schema::tags::dsl::*;

        let mut conn = pool.get()?;

        let res: Vec<Tag> = tags.load(&mut conn)?;

        Ok(res)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let res = JsonResponse {
        success: true,
        message: "Tags retrieved successfully".to_string(),
        data: Some(serde_json::to_value(tags).unwrap()),
    };
    Ok(HttpResponse::Ok().json(res))
}

async fn create_tag(pool: Data<DbPool>, data: Json<NewTag>) -> actix_web::Result<impl Responder> {
    let new_tag: Tag = web::block(move || -> Result<Tag, DbError> {
        use crate::db::schema::tags::dsl::*;

        let mut conn = pool.get()?;

        let new_tag = Tag {
            id: Uuid::new_v4().to_string(),
            title: data.title.clone(),
        };

        diesel::insert_into(tags)
            .values(&new_tag)
            .execute(&mut conn)?;

        Ok(new_tag)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let res: JsonResponse = JsonResponse {
        success: true,
        message: "Tag created successfully".to_string(),
        data: Some(serde_json::to_value(new_tag).unwrap()),
    };
    Ok(HttpResponse::Ok().json(res))
}

async fn update_tag(pool: Data<DbPool>, data: Json<Tag>) -> actix_web::Result<impl Responder> {
    let updated_tag: Tag = web::block(move || -> Result<Tag, DbError> {
        use crate::db::schema::tags::dsl::*;

        let mut conn = pool.get()?;

        let updated_tag: Tag = data.into_inner();

        diesel::update(tags.find(&updated_tag.id))
            .set(&updated_tag)
            .execute(&mut conn)?;

        Ok(updated_tag)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let res: JsonResponse = JsonResponse {
        success: true,
        message: "Updated Tag successfully".to_string(),
        data: Some(serde_json::to_value(updated_tag).unwrap()),
    };
    Ok(HttpResponse::Ok().json(res))
}

async fn delete_tag(pool: Data<DbPool>, uid: Path<Uuid>) -> actix_web::Result<impl Responder> {
    web::block(move || -> Result<(), DbError> {
        use crate::db::schema::tags::dsl::*;

        let mut conn = pool.get()?;

        diesel::delete(tags.filter(id.eq(uid.into_inner().to_string()))).execute(&mut conn)?;

        Ok(())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let res: JsonResponse = JsonResponse {
        success: true,
        message: "Deleted Tag Successfully".to_string(),
        data: None,
    };
    Ok(HttpResponse::Ok().json(res))
}

pub fn register(config: &mut ServiceConfig) {
    config.service(
        web::resource("")
            .route(web::get().to(get_all_tags))
            .route(web::post().to(create_tag))
            .route(web::put().to(update_tag)),
    );

    config.service(web::resource("/{uid}").route(web::delete().to(delete_tag)));
}
