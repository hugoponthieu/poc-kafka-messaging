use crate::repositories::{messages_repo::MessagesRepoTrait, Repositories};

pub struct Recorder {
    pub repositories: Repositories,
}

pub trait RecorderBuilder: Send + Sync {
    fn build(repositories: Repositories) -> Recorder;
}

pub trait RecorderMethods: Send + Sync {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl RecorderBuilder for Recorder {
    fn build(repositories: Repositories) -> Recorder {
        Recorder { repositories }
    }
}

impl RecorderMethods for Recorder {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let message_repo = self.repositories.messages_repo.read().await;
        let (stream, _) = message_repo.get_topic_message_stream();

        tokio::spawn(async {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });
        Ok(())
    }
}
