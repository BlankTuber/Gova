#[macro_use] extern crate rocket;

mod db;
mod routes;
mod models;
mod services;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    // Initialize MongoConnection
    let mongo_conn = db::mongo::MongoConnection::new(
        "mongodb://localhost:27017", 
        "user-space-api"
    ).await.expect("Failed to connect to MongoDB");

    // Ping MongoDB to ensure connectivity
    mongo_conn.ping().await.expect("MongoDB ping failed");

    // Create an instance of AuthService using the database from MongoConnection
    let auth_service = services::auth::AuthService::new(mongo_conn.database.clone());

    // Build the Rocket instance
    rocket::build()
        .manage(mongo_conn)
        .manage(auth_service)
        .mount("/", routes![index, routes::user::create_user, routes::user::get_user])
}