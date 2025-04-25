use std::collections::HashMap;

use super::adapter::DataSourceAdapter;

pub struct DataSourceManager {
    adapters: HashMap<String, Box<dyn DataSourceAdapter>>,
}