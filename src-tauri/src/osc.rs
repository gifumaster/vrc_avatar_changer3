use std::net::UdpSocket;

use rosc::{encoder, OscMessage, OscPacket, OscType};
const OSC_ADDRESS: &str = "/avatar/change";
const OSC_EYE_HEIGHT_ADDRESS: &str = "/avatar/eyeheight";

pub struct OscClient;

impl OscClient {
    pub fn new() -> Self {
        Self
    }

    pub fn switch_avatar(&self, avatar_id: &str, host: &str, port: u16) -> Result<(), String> {
        self.send_message(
            host,
            port,
            OSC_ADDRESS,
            vec![OscType::String(avatar_id.to_string())],
        )
    }

    pub fn set_avatar_eye_height(&self, eye_height_meters: f32, host: &str, port: u16) -> Result<(), String> {
        self.send_message(
            host,
            port,
            OSC_EYE_HEIGHT_ADDRESS,
            vec![OscType::Float(eye_height_meters)],
        )
    }

    fn send_message(
        &self,
        host: &str,
        port: u16,
        address: &str,
        args: Vec<OscType>,
    ) -> Result<(), String> {
        let packet = OscPacket::Message(OscMessage {
            addr: address.to_string(),
            args,
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
