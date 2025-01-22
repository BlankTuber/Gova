use mongodb::Client;
use mongodb::options::ClientOptions;
use mongodb::bson::doc;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

async fn connect() -> Result<Client, mongodb::error::Error> {
    let uri = "mongodb://localhost:27017/user-space-api";  // Corrected port
    let options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(options)?;
    
    client.database("admin").run_command(doc! {"ping": 1}).await?;
    println!("Successfully connected to MongoDB!");
    Ok(client)
}


#[launch]
async fn rocket() -> _ {
    let client = match connect().await {
        Ok(client) => client,
        Err(_) => panic!("Could not connect to MongoDB"),
    };
    rocket::build()
        .manage(client)  // Store client in Rocket's managed state
        .mount("/", routes![index])
}
