use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use mongodb::bson::doc;

pub struct MongoConnection {
    pub client: Client,
    pub database: Database
}

impl MongoConnection {
    pub async fn new(connection_string: &str, db_name: &str) -> Result<Self, mongodb::error::Error> {
        let client_options = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(db_name);

        Ok(Self {client, database})
    }

    pub async fn ping(&self) -> Result<(), mongodb::error::Error> {
        self.client
            .database("admin")
            .run_command(doc! {"ping": 1})
            .await?;
        self.database
            .run_command(doc! {"ping": 1})
            .await?;
        Ok(())
    }
}