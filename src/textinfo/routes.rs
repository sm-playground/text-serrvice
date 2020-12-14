use crate::textinfo::TextInfo;

use crate::api_error::ApiError;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

/*
use crate::schema;
use crate::textinfo::InsertableTextInfo;
use diesel::{self, prelude::*};


#[get("/texts")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        TextInfo {
            id: 1,
            token: "abc".to_string(),
            text: "abracadabra".to_string(),
        },
        TextInfo {
            id: 2,
            token: "def".to_string(),
            text: "abracadabra".to_string(),
        },
    ])
}

#[get("/texts/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(TextInfo {
        id: 1,
        token: "abc".to_string(),
        text: "abracadabra".to_string(),
    })
}

#[delete("/texts/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Deleted"}))
}
*/
#[get("/texts")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let all_text_info = TextInfo::find_all()?;
    Ok(HttpResponse::Ok().json(all_text_info))
}

#[get("/texts/{id}")]
async fn find(id: web::Path<i64>) -> Result<HttpResponse, ApiError> {
    let user = TextInfo::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/texts")]
async fn create(user: web::Json<TextInfo>) -> impl Responder {
    HttpResponse::Created().json(user.into_inner())
}

#[put("/texts/{id}")]
async fn update(user: web::Json<TextInfo>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner())
}

#[delete("/texts/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, ApiError> {
    let num_deleted = TextInfo::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
