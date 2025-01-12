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
use crate::util::env_manager::require_item;
use crate::util::error_handling::application_error::ApplicationError;
use crate::util::middleware::authenticate::Authenticate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ctx = build_app_context().await;

    if let Err(e) = ctx {
        return Err(std::io::Error::new(ErrorKind::Other, format!("Could not initialize application context: {0}", e)));
    }

    let context = ctx.unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let server_port = require_item("SERVER_PORT").unwrap().parse::<u16>().unwrap();

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
        .bind(("0.0.0.0", server_port))?
        .run()
        .await
}

async fn build_app_context() -> Result<AppContext, ApplicationError> {
    let connection_string = require_item("DATABASE_URL")?;
    let db = Database::connect(connection_string).await;
    if db.is_err() {
        return Err(ApplicationError::new(format!("Failed to create db connection: {0}", db.unwrap_err()).as_str()))
    }

    Ok(AppContext {
        db: db.unwrap()
    })
}