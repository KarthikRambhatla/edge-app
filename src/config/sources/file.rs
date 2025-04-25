use crate::config::{source::ConfigSource, Config};

pub struct FileConfigSource {
    file_path: String,
}

impl FileConfigSource {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl ConfigSource for FileConfigSource {
    fn load_config(&self) -> Vec<Config> {
        todo!()
    }
    
    fn watch_config(&self) -> Vec<Config> {
        todo!()
    }
}
