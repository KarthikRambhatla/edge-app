use super::sink::DataSink;

pub struct DataSinkManager {
    pub data_sinks: Vec<Box<dyn DataSink>>,
}