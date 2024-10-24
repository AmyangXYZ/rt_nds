use crate::packet::{Packet, PacketType, PACKET_SIZE};
use std::net::UdpSocket;

pub struct Client {
    pub id: u64,
    server_address: String,
    socket: UdpSocket,
}

impl Client {
    pub fn new(id: u64, server_address: &str) -> Self {
        println!("Client {} created", id);
        Self {
            id,
            server_address: server_address.to_string(),
            socket: UdpSocket::bind("[::]:0").unwrap(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Vec<u8>> {
        let mut packet = Packet::default();
        packet.ptype = PacketType::Get as u8;
        packet.data_name = name.to_string();
        let _ = self
            .socket
            .send_to(&packet.to_buffer(), &self.server_address);
        let mut buf = [0; PACKET_SIZE];
        let (amt, _) = self.socket.recv_from(&mut buf).unwrap();
        let packet = Packet::from_buffer(&buf[..amt]);
        Some(packet.data)
    }

    pub fn set(&self, name: &str, data: Vec<u8>) -> bool {
        let mut packet = Packet::default();
        packet.data_name = name.to_string();
        packet.ptype = PacketType::Set as u8;
        packet.data = data;
        let _ = self
            .socket
            .send_to(&packet.to_buffer(), &self.server_address);
        true
    }
}
