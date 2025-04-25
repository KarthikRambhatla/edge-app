// configuration needed to issue a request
use crate::{data_sink::sink_type::SinkType, data_source::protocol::Protocol};

pub struct Config {
    pub(crate) protocol: Protocol,
    pub(crate) endpoint: String,
    pub(crate) request: String,
    pub(crate) polling_interval: u32,
    pub sink_config: SinkConfig
}

// Here we store all the details that are needed to publish to a sink,
// for example, it might be to a messaging queue
pub struct SinkConfig {
    pub sink_type: SinkType,
    pub destination: String,
    // This should be depending on the enum, rust has a way to do this idiomatically
}