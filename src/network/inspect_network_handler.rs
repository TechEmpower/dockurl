use curl::easy::{Handler, WriteError};
use serde_json::Value;
use std::borrow::Cow;

pub struct InspectNetworkHandler<H: Handler> {
    pub network_id: Option<String>,
    pub error_message: Option<String>,
    accumulator: Vec<u8>,
    log_handler: H,
}
impl<H: Handler> InspectNetworkHandler<H> {
    pub fn new(log_handler: H) -> Self {
        Self {
            network_id: None,
            error_message: None,
            accumulator: vec![],
            log_handler,
        }
    }
    pub fn body(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.accumulator)
    }
}
impl<H: Handler> Handler for InspectNetworkHandler<H> {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.accumulator.extend_from_slice(data);
        self.log_handler.write(data)?;

        if let Ok(logs) = std::str::from_utf8(&data) {
            for line in logs.lines() {
                if !line.trim().is_empty() {
                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        if !json["Id"].is_null() {
                            let mut network_id = json["Id"].as_str().unwrap();
                            network_id = &network_id[0..12];
                            self.network_id = Some(network_id.to_string());
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
