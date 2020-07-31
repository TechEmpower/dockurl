use curl::easy::{Handler, WriteError};
use serde_json::Value;

pub struct CreateContainerHandler<H: Handler> {
    pub container_id: Option<String>,
    pub error_message: Option<String>,
    handler: H,
}
impl<H: Handler> CreateContainerHandler<H> {
    pub fn new(handler: H) -> Self {
        Self {
            container_id: None,
            error_message: None,
            handler,
        }
    }
}
impl<H: Handler> Handler for CreateContainerHandler<H> {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.handler.write(data)?;

        if let Ok(logs) = std::str::from_utf8(data) {
            for line in logs.lines() {
                if !line.trim().is_empty() {
                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        if !json["Id"].is_null() {
                            let mut container_id = json["Id"].as_str().unwrap();
                            container_id = &container_id[0..12];
                            self.container_id = Some(container_id.to_string());
                        } else if !json["message"].is_null() {
                            // fixme - this APPEARS to be how docker communicates error messages.
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
