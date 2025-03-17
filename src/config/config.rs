// configuration needed to issue a request
use crate::data_source::protocol::Protocol;

pub struct Config {
    pub(crate) protocol: Protocol,
    pub(crate) endpoint: String,
    pub(crate) request: String,
    pub(crate) response_transform: String,
    pub(crate) polling_interval: u32,
}

