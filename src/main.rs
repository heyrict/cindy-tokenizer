#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate lazy_static;

mod actions;
mod models;
mod schema;
mod tokenize;

use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

#[get("/tokenize/{puzzle_id}")]
async fn tokenize_puzzle(
    pool: web::Data<DbPool>,
    info: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let puzzle_id = info.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let tokens_list: Vec<Vec<tokenize::Token>> =
        web::block(move || actions::get_puzzle_tokens(puzzle_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    Ok(HttpResponse::Ok().json(tokens_list))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    dotenv().ok();

    let bind = format!(
        "{}:{}",
        env::var("BIND").expect("BIND must be set"),
        env::var("PORT").expect("PORT must be set")
    );
    let pool = establish_connection();

    HttpServer::new(move || App::new().data(pool.clone()).service(tokenize_puzzle))
        .bind(&bind)?
        .run()
        .await
}
