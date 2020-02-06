#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware, web, App, HttpServer};
use std::env;

use db::DbPool;

mod api;
mod db;
mod graphql;
mod models;
mod schema;

pub type Error = Box<dyn std::error::Error>;
pub type AppResult<T> = Result<T, Error>;

diesel_migrations::embed_migrations!("migrations");

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    // DB
    let pool = db::establish_connection();
    embedded_migrations::run(&pool.get().unwrap()).expect("Failed to run migration");

    // API
    let api_port = env::var("API_PORT").unwrap_or_else(|_| String::from("3000"));
    let api_url = format!("0.0.0.0:{}", api_port);

    println!("Started http server: {}", api_url);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(api::register)
    })
    .bind(api_url)
    .expect("Failed to start server")
    .run()
    .await
}
