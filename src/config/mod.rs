use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AppConfig {
    #[arg(long, env = "KAFKA_HOST", default_value = "localhost:9092")]
    pub kafka_host: String,

    #[arg(long, env = "HTTP_HOST", default_value = "localhost")]
    pub http_host: String,

    #[arg(long, env = "HTTP_PORT", default_value_t = 8080)]
    pub http_port: u16,
}

impl AppConfig {
    pub fn builder() -> AppConfig {
        AppConfig {
            kafka_host: String::new(),
            http_host: String::new(),
            http_port: 0,
        }
    }
    
    pub fn make() -> AppConfig {
        let args = AppConfig::parse();
        let config = AppConfig {
            kafka_host: args.kafka_host,
            http_host: args.http_host,
            http_port: args.http_port,
        };
        config.log_config();
        config
    }

    pub fn set_kafka_host(&mut self, host: String) {
        self.kafka_host = host;
    }

    pub fn set_http_host(&mut self, host: String) {
        self.http_host = host;
    }

    pub fn set_http_port(&mut self, port: u16) {
        self.http_port = port;
    }

    pub fn default() -> Self {
        AppConfig {
            kafka_host: "localhost:9092".to_string(),
            http_host: "localhost".to_string(),
            http_port: 8080,
        }
    }

    pub fn log_config(&self) {
        tracing::info!("Kafka Host: {}", self.kafka_host);
        tracing::info!("HTTP Host: {}", self.http_host);
        tracing::info!("HTTP Port: {}", self.http_port);
    }
}
