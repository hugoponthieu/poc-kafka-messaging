use std::error::Error;

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

pub struct MongoDBClient {
    pub host: String,
    pub client: mongodb::Client,
}

pub trait MongoDBClientTrait: Send + Sync {
    async fn build(host: &str) -> Result<MongoDBClient, Box<dyn Error>>;
}

impl MongoDBClientTrait for MongoDBClient {
    async fn build(host: &str) -> Result<MongoDBClient, Box<dyn Error>> {
        let mut client_options = ClientOptions::parse(host).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options)?;
        match client
            .database("beep")
            .run_command(doc! { "ping": 1 })
            .await
        {
            Ok(_) => tracing::info!("Connected to MongoDB"),
            Err(e) => {
                tracing::error!("Failed to connect to MongoDB: {}", e);
                return Err(Box::new(e));
            }
        };
        Ok(MongoDBClient {
            host: host.to_string(),
            client,
        })
    }
}
