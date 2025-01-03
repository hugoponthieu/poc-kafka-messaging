use std::sync::Arc;

use tokio_stream::StreamExt;

use crate::repositories::{
    messages_repo::MessagesRepoTrait,
    Repositories,
};

pub struct Recorder {
    pub repositories: Repositories,
}

pub trait RecorderBuilder: Send + Sync {
    fn build(repositories: Repositories) -> Recorder;
}

pub trait RecorderMethods: Send + Sync {
    async fn start(self) -> Result<(), Box<dyn std::error::Error+ Send + Sync>>;
}

impl RecorderBuilder for Recorder {
    fn build(repositories: Repositories) -> Recorder {
        Recorder { repositories }
    }
}

impl RecorderMethods for Recorder {
    async fn start(self) -> Result<(), Box<dyn std::error::Error+ Send + Sync>> {
        tracing::info!("preparing recorder");
        let recorder = Arc::new(self);

        let message_repo = recorder.repositories.messages_repo.read().await;
        tracing::info!("got recorder");

        let (mut stream, _) = message_repo.get_topic_message_stream();
        
        let repo = recorder.clone().repositories.messages_repo.clone();
        tokio::spawn(async move {
            tracing::debug!("Recorder ready");
            while let Some(message) = stream.next().await {
                let msg = match message {
                    Ok(msg) => msg,
                    Err(_) => continue,
                };
                tracing::debug!("got message");
                
                if let Err(e) = repo.read().await.save_message(msg).await {
                      tracing::error!("Failed to save message: {}", e);
                      continue;
                  }           
            }
            tracing::debug!("got message");

        });
        tracing::info!("Recorder started");
        Ok(())
    }
}
