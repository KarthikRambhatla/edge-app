// use std::net::UdpSocket;
// use std::str;
mod data_source;
mod config; // This tells Rust to look for either config.rs or config/mod.rs
use crate::config::Config; //This brings Config type
use config::sources::in_mem::InMemory;

fn main() {
    // let wiz_bulb_ip = "192.168.0.104:38899"; // Replace with your bulb's IP

    // // Create a UDP socket
    // let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    // // Wiz API request to get bulb status
    // let request = r#"{"method": "getPilot", "id": 1}"#;
    
    // // Send the request to the bulb
    // socket.send_to(request.as_bytes(), wiz_bulb_ip).expect("Failed to send request");

    // // Receive response from the bulb
    // let mut buf = [0; 1024];
    // let (amt, _src) = socket.recv_from(&mut buf).expect("Failed to receive response");

    // // Convert response to string and print
    // let response = str::from_utf8(&buf[..amt]).expect("Failed to parse response");
    // println!("Bulb Response: {}", response);

    let wiz_bulb_config = Config {
        protocol: data_source::protocol::Protocol::UDP,
        endpoint: "192.168.0.104:38899".to_string(),
        request: r#"{"method": "getPilot", "id": 1}"#.to_string(),
        response_transform: "json".to_string(),
        polling_interval: 0,
    };

    let config_source = InMemory {
        configs: vec![wiz_bulb_config],
    };
    
    println!("Hello, world!");
}
