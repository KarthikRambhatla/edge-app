use super::adapter::DataSourceAdapter;

pub struct DataSourceManager {
    adapters: Vec<Box<dyn DataSourceAdapter>>,
}

impl DataSourceManager {
    pub fn new() -> Self {
        Self {
            adapters: Vec::new(),
        }
    }
}
