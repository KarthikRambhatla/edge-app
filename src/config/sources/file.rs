pub struct FileConfigSource {
    file_path: String,
}

impl FileConfigSource {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl ConfigSource for FileConfigSource {
    fn load_config(&self) -> Result<Config, ConfigError> {
        let file = std::fs::File::open(&self.file_path)?;
        let reader = std::io::BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
