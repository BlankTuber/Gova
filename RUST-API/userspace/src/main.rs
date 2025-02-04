#[macro_use] extern crate rocket;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod middleware;
mod models;
mod routes;
mod utils;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to Postgres");

    rocket::build()
        .manage(pool)
        .mount("/api/auth", routes![
            routes::auth::register::register,
            // Add more routes
        ])
        .mount("/api/users", routes![
            // User routes
        ])
}