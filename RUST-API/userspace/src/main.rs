#[macro_use] extern crate rocket;

mod db;
mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[launch]
async fn rocket() -> _ {
    // Use .expect() or proper error propagation
    let mongo_conn = db::mongo::MongoConnection::new(
        "mongodb://localhost:27017", 
        "user-space-api"
    ).await.expect("Failed to connect to MongoDB");

    // Properly await the ping and handle potential errors
    mongo_conn.ping().await.expect("MongoDB ping failed");

    rocket::build()
        .manage(mongo_conn)
        .mount("/", routes![index, routes::user::get_user])
}
