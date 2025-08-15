use std::thread;

use crate::{config::Config, data_sink::manager::DataSinkManager, data_source::manager::DataSourceManager};

pub struct App {
    data_source_manager: DataSourceManager,
    data_sink_manager: DataSinkManager,
    app_config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        // Initialize with configuration
        let data_source_manager = DataSourceManager::new(&config);
        let data_sink_manager = DataSourceManager::new(&config);
        let app_config = config;
        Self {
            data_source_manager,
            data_sink_manager,
            app_config,
        }
    }

    pub fn start(&mut self) {
        // polling loop
        loop {
            // Get data from source
            let data = self.data_source_manager.get_data(&self.config);
            
            // Send to configured sinks
            self.data_sink_manager.publish(data);
            
            thread::sleep(Duration::from_secs(self.config.polling_interval));
        }
    }
}