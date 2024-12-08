use actix_web::{HttpResponse, post, Responder, Scope, web};
use web::Data;
use crate::model::credentials::Credentials;
use crate::util::app_context::AppContext;
use crate::core::auth_service;
use crate::model::new_user::NewUser;
use crate::model::password_update::PasswordUpdate;


#[post("/login")]
async fn login(body: web::Json<Credentials>, ctx: Data<AppContext>) -> impl Responder {
    return match auth_service::login(&body.user_name, &body.password, &ctx).await {
        Err(e) => HttpResponse::Unauthorized().body("unauthorized"),
        Ok(token) => HttpResponse::Ok().body(token)
    }
}

#[post("/register")]
async fn register(body: web::Json<NewUser>, ctx: Data<AppContext>) -> impl Responder {
    return match auth_service::register(&body.name_first, &body.name_last, &body.email, &body.password, &ctx).await {
        Err(e) => HttpResponse::Unauthorized().body("unauthorized:".to_string() + &*e.to_string()),
        Ok(token) => HttpResponse::Ok().body(token)
    }
}

#[post("/update")]
async fn update_password(body: web::Json<PasswordUpdate>, ctx: Data<AppContext>) -> impl Responder {
    match auth_service::update(&body.email, &body.old_password, &body.new_password, &ctx).await {
        Err(e) => HttpResponse::Unauthorized().body("unauthorized:".to_string() + &*e.to_string()),
        Ok(token) => HttpResponse::Ok().body(token)
    }
}

#[post("/reset")]
async fn reset_password(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub fn get_service() -> Scope {
    web::scope("/auth")
        .service(login)
        .service(register)
        .service(update_password)
        .service(reset_password)
}
