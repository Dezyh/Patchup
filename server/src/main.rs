#![allow(dead_code)]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

extern crate uuid;
extern crate jsonwebtoken;

use actix_web::{middleware::Logger, App, HttpServer};
use std::{env, io};

mod error;
mod config;
mod routes;
mod models;
mod schema;
mod services;
mod constants;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Couldn't find .env file");
    env_logger::init();

    let server_host = env::var("SERVER_HOST").expect("SERVER_HOST must be set");
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let server_url = format!("{}:{}", &server_host, &server_port);

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = config::db::migrate_and_config_db(&db_url);

    let server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(Logger::default())
            .configure(config::routes::config_routes)
    })
    .bind(&server_url)?
    .run();

    server.await
}
