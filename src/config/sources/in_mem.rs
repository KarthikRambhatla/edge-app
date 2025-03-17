use crate::config::{source::ConfigSource, Config};
pub struct InMemory {
    pub configs: Vec<Config>
}

impl ConfigSource for InMemory {
    fn load_config(&self) -> Vec<Config> {
        todo!()
    }

    fn watch_config(&self) -> Vec<Config> {
        todo!()
    }
}