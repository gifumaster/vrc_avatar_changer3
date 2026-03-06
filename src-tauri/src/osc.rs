use std::net::UdpSocket;

use rosc::{encoder, OscMessage, OscPacket, OscType};
const OSC_ADDRESS: &str = "/avatar/change";

pub struct OscClient;

impl OscClient {
    pub fn new() -> Self {
        Self
    }

    pub fn switch_avatar(&self, avatar_id: &str, host: &str, port: u16) -> Result<(), String> {
        let packet = OscPacket::Message(OscMessage {
            addr: OSC_ADDRESS.to_string(),
            args: vec![OscType::String(avatar_id.to_string())],
        });

        let bytes = encoder::encode(&packet).map_err(|error| error.to_string())?;
        let socket = UdpSocket::bind("127.0.0.1:0").map_err(|error| error.to_string())?;
        let target = format!("{host}:{port}");

        socket
            .send_to(&bytes, target)
            .map_err(|error| error.to_string())?;

        Ok(())
    }
}
