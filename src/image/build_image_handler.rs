use curl::easy::{Handler, WriteError};
use serde_json::Value;

pub struct BuildImageHandler<H: Handler> {
    pub image_id: Option<String>,
    pub error_message: Option<String>,
    handler: H,
}
impl<H: Handler> BuildImageHandler<H> {
    pub fn new(handler: H) -> Self {
        Self {
            image_id: None,
            error_message: None,
            handler,
        }
    }
}
impl<H: Handler> Handler for BuildImageHandler<H> {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.handler.write(data)?;

        if let Ok(logs) = std::str::from_utf8(data) {
            for line in logs.lines() {
                if !line.trim().is_empty() {
                    // Docker is sending us lines of json encoded strings on every write.
                    // These look like:
                    // {"stream":" ---\u003e Using cache\n"}
                    // I don't know enough about this API to state definitively that the
                    // "stream" values are all we care about, but it seems likely. Other
                    // keys exist, such as:
                    // {"aux":{"ID":"sha256:e821df6f41ad85f08c5fa08a228a34e164d93995e89be2d0d5edb9206a715347"}}
                    // which looks like the id of the image that was built. Likely, we
                    // neither care nor need to log it.
                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        if !json["aux"].is_null() {
                            let line = json["aux"]["ID"].as_str().unwrap();
                            // fixme - This is a hack to remove "sha256:" from the string, but
                            //  it may not always use sha256, so this should be done right.
                            let sha = &line[7..];
                            self.image_id = Some(sha.to_string());
                        } else if !json["error"].is_null() {
                            let error = json["error"].as_str().unwrap().to_string();
                            self.error_message = Some(error);
                        } else if !json["message"].is_null() {
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
