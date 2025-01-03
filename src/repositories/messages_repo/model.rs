use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub _id: Option<String>,
    pub content: String, 
}

impl AsRef<str> for Message {
    fn as_ref(&self) -> &str {
        &self.content
    }
}
