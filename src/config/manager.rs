// config source manager should know what the sources are, current config, notifier to notify other parts of code about change
use crate::config::source::ConfigSource;
use crate::config::config::Config;

pub struct ConfigSourceManager {
    pub sources: Vec<Box<dyn ConfigSource>>, // since this is a trait, we made dyn but unless boxed, i couldn't keep in vec - why?
    pub current_config: Vec<Config>,
    pub notifier: String // need to check what this should be
} 

impl ConfigSourceManager {
    pub fn new(sources: Vec<Box<dyn ConfigSource>>) -> Self {
        ConfigSourceManager {
            sources: sources,
            current_config: Vec::new(),
            notifier: String::new()
        }
    }
    pub fn add_source(&mut self, source: Box<dyn ConfigSource>) {
        self.sources.push(source);
        println!("Source added");
    }

    pub fn load_all(&mut self) {
        self.current_config.clear();
        for source in &self.sources {
            for config in source.load_config() {
                self.current_config.push(config);
            }
            //need to think something better, why loop and push?
        }
        println!("Configs loaded");
    }

    pub fn watch_all(&self) {
        for source in &self.sources {
            source.watch_config();
        }
        println!("For now nothing happens")
    }

    pub async fn run(&mut self) {
        self.load_all();
        self.watch_all();
    }
}