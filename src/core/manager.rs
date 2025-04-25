//TODO: This name needs to be changed. Not everything is manager, adapter - right?
pub struct CoreManager {
    data_source_manager: DataSourceManager,
    data_sink_manager: DataSinkManager,
    config: Config,
}

impl CoreManager {
    pub fn new(config: Config) -> Self {
        // Initialize with configuration
    }

    pub fn start(&mut self) {
        // Start polling/processing loop
        loop {
            // 1. Get data from source
            let data = self.data_source_manager.get_data(&self.config);
            
            // 2. Process data if needed
            
            // 3. Send to configured sinks
            self.data_sink_manager.publish(data);
            
            // 4. Wait for next interval
            thread::sleep(Duration::from_secs(self.config.polling_interval));
        }
    }
}