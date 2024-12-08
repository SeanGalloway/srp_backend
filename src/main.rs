mod controller;
mod model;
mod core;
mod data;
mod util;

use std::fmt::{Debug, format};
use std::io::ErrorKind;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use env_logger::Env;
use sea_orm::{Database, DatabaseConnection};
use crate::controller::auth_controller;
use crate::controller::note_controller;
use crate::util::app_context;
use crate::util::app_context::AppContext;
use crate::util::middleware::authenticate::Authenticate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ctx = build_app_context().await;

    if let Err(e) = ctx {
        return Err(std::io::Error::new(ErrorKind::Other, format!("Could not initialize application context: {0}", e)));
    }

    let context = ctx.unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600))
            .app_data(web::Data::new(context.clone()))
            .service(auth_controller::get_service())
            .service(note_controller::get_service().wrap(Authenticate))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn build_app_context() -> Result<AppContext, String> {
    let db = Database::connect("postgres://local_admin:admin_pwd@localhost:32760/srp_auth").await;
    if db.is_err() {
        return Err(format!("Failed to create db connection: {0}", db.unwrap_err()))
    }

    Ok(AppContext {
        db: db.unwrap()
    })
}