use std::error::Error;

use mongodb::{options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

pub struct MongoDBClient {
    pub host: String,
    pub client: mongodb::Client,
}

pub trait MongoDBClientTrait: Send + Sync {
    async fn build(host: &str) -> Result<MongoDBClient, Box<dyn Error>>;
}

impl MongoDBClientTrait for MongoDBClient {
    async fn build(host: &str) -> Result<MongoDBClient, Box<dyn Error>>  {
        let mut client_options = ClientOptions::parse(host).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options)?;        
        Ok(MongoDBClient {
            host: host.to_string(),
            client,
        })
    }
}