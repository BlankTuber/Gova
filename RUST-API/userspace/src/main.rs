#[macro_use] extern crate rocket;

use std::env;
mod config;
use config::database;

mod models;
mod routes;
mod controllers;
mod middleware;
mod utils;

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pool = database::init_pool(&database_url);


    // Build the Rocket instance
    rocket::build()
        .manage(pool)
        .mount("/api", routes::routes())
}