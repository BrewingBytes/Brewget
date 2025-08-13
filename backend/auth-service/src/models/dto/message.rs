use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    message: String,
}

impl Message {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
