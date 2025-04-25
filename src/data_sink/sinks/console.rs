use crate::data_sink::sink::DataSink;

pub struct ConsoleSink;
impl DataSink for ConsoleSink {
    fn connect(&self) -> Result<(), String> {
        Ok(())
    }
    fn publish(&self, data: &str) -> Result<(), String> {
        println!("Data: {}", data);
        Ok(())
    }
}