use crate::packet::{Packet, PacketType, MAX_DATA_SIZE, PACKET_SIZE};
use crc32fast;
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

    pub fn set(&self, name: &str, data: Vec<u8>, expiry: u64) -> bool {
        let size = data.len() as u32;
        let num_chunks = (size as f32 / MAX_DATA_SIZE as f32).ceil() as u16;
        for i in 0..num_chunks {
            let mut packet = Packet::default();
            packet.ptype = PacketType::Set as u8;
            packet.data_name = name.to_string();
            packet.data_expiry = expiry;
            packet.data_size = size;
            packet.data_chunk_num = num_chunks;
            packet.data_chunk_id = i;
            let start = (i as usize * MAX_DATA_SIZE) as usize;
            let end = ((i as usize + 1) * MAX_DATA_SIZE) as usize;
            let chunk = data[start..data.len().min(end)].to_vec();
            packet.data_checksum = crc32fast::hash(&chunk);
            println!("Chunk {} checksum: {}", i, packet.data_checksum);
            packet.data = chunk;

            let _ = self
                .socket
                .send_to(&packet.to_buffer(), &self.server_address);
        }
        true
    }
}
