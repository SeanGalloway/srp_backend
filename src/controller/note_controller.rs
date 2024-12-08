use actix_web::{HttpResponse, get, post, Responder, web, Scope, put, delete, HttpRequest, HttpMessage};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use crate::core::auth_service;
use crate::core::note_service::get_all;
use crate::model::credentials::Credentials;
use crate::model::user_claim::UserClaim;
use crate::util::app_context::AppContext;

#[get("/all")]
async fn retrieve_all(req: HttpRequest, ctx: Data<AppContext>) -> impl Responder {
    if let Some(claims) = req.extensions().get::<UserClaim>() {
        match get_all(claims.id, &ctx).await {
            Ok(notes) => HttpResponse::Ok().json(notes),
            Err(_) => HttpResponse::InternalServerError().body("Error retrieving notes")
        }
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

#[get("/{id}")]
async fn retrieve_one(body: web::Json<Credentials>, ctx: Data<AppContext>) -> impl Responder {
    HttpResponse::Ok().body("token")
}

#[post("/")]
async fn create(body: web::Json<Credentials>, ctx: Data<AppContext>) -> impl Responder {
    HttpResponse::Ok().body("token")
}

#[put("/{id}")]
async fn update(body: web::Json<Credentials>, ctx: Data<AppContext>) -> impl Responder {
    HttpResponse::Ok().body("token")
}

#[delete("/")]
async fn delete(body: web::Json<Credentials>, ctx: Data<AppContext>) -> impl Responder {
    HttpResponse::Ok().body("token")
}

pub fn get_service() -> Scope {
    web::scope("/note")
        .service(retrieve_all)
        .service(retrieve_one)
        .service(create)
        .service(update)
        .service(delete)
}