//TODO: error handling
pub trait DataSink {
    fn connect(&self) -> Result<(), String>;
    fn publish(&self, data: &str) -> Result<(), String>;
}