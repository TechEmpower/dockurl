use curl::easy::{Handler, WriteError};
use std::borrow::Cow;

pub struct ConnectContainerToNetworkHandler<H: Handler> {
    pub accumulator: Vec<u8>,
    log_handler: H,
}
impl<H: Handler> ConnectContainerToNetworkHandler<H> {
    pub fn new(log_handler: H) -> Self {
        Self {
            accumulator: vec![],
            log_handler,
        }
    }
    pub fn body(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.accumulator)
    }
}
impl<H: Handler> Handler for ConnectContainerToNetworkHandler<H> {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.accumulator.extend_from_slice(data);
        self.log_handler.write(data)?;

        Ok(data.len())
    }
}
