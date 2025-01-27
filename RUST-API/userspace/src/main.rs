#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    // Build the Rocket instance
    rocket::build()
        .mount("/", routes![index])
}