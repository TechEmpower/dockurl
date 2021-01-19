use curl::easy::{Handler, WriteError};

pub struct DeleteContainerHandler<H: Handler> {
    pub error_message: Option<String>,
    pub accumulator: Vec<u8>,
    handler: H,
}
impl<H: Handler> DeleteContainerHandler<H> {
    pub fn new(handler: H) -> Self {
        Self {
            accumulator: vec![],
            error_message: None,
            handler,
        }
    }
}
impl<H: Handler> Handler for DeleteContainerHandler<H> {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.handler.write(data)?;
        self.accumulator.extend_from_slice(data);

        Ok(data.len())
    }
}
