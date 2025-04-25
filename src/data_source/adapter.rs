//TODO: we should return custom errors, error handling needs to be dealt 
// for now returning string to focus on core func.
pub trait DataSourceAdapter {
    fn connect(&self) -> Result<(), String>;
    fn execute_request(&self, request: &str) -> Result<String, String>;
}