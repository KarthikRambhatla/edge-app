// a config source should be able to load config and watch for changes
use crate::config::Config;

pub trait ConfigSource {
    fn load_config(&self) -> Vec<Config>;
    fn watch_config(&self) -> Vec<Config>; // may be we just need to give the delta, and tell whether add or remove
}