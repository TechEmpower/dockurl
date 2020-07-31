use curl::easy::{Handler, WriteError};
use serde_json::Value;

pub struct StartContainerHandler<H: Handler> {
    pub image_id: Option<String>,
    pub error_message: Option<String>,
    handler: H,
}
impl<H: Handler> StartContainerHandler<H> {
    pub fn new(handler: H) -> Self {
        Self {
            image_id: None,
            error_message: None,
            handler,
        }
    }
}
impl<H: Handler> Handler for StartContainerHandler<H> {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.handler.write(data)?;

        if let Ok(logs) = std::str::from_utf8(data) {
            for line in logs.lines() {
                if !line.trim().is_empty() {
                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        if !json["message"].is_null() {
                            let error = json["message"].as_str().unwrap().to_string();
                            self.error_message = Some(error);
                        }
                    }
                }
            }
        }

        Ok(data.len())
    }
}
