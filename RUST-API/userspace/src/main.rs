#[macro_use] extern crate rocket;

use rocket::serde::json::{Json, json};
use rocket::serde::json::Value;
use rocket::Request;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod middleware;
mod models;
mod routes;
mod utils;
use middleware::cors::CORS;

#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": 404,
        "error": {
            "message": "The path you're looking for seems to be missing in the void.",
            "details": "Check the URL and try again, or navigate back to safety.",
            "code": "RESOURCE_NOT_FOUND"
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[catch(500)]
fn internal_error(req: &Request) -> Json<Value> {
    Json(json!({
        "status": 500,
        "error": {
            "message": "Something went wrong on our end.",
            "details": "Our team has been notified and is working on it.",
            "code": "INTERNAL_SERVER_ERROR",
            "path": req.uri().path().to_string()
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to Postgres");

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let config = rocket::Config::figment()
        .merge(("secret_key", secret.as_bytes()));

    rocket::custom(config)
        .manage(pool)
        .attach(CORS)
        .mount("/api/auth", routes::auth_routes())
        .mount("/api/admin", routes::admin_routes())
        .mount("/api/users", routes::user_routes())
        .register("/", catchers![not_found, internal_error])
}