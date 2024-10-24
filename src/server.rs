use crate::cache::Cache;
use crate::packet::{Packet, PacketType, PACKET_SIZE, TOKEN};
use std::net::UdpSocket;
use std::time::Duration;
pub struct Server {
    cache: Cache,
    socket: UdpSocket,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Self {
            cache: Cache::new(),
            socket: UdpSocket::bind(address).unwrap(),
        }
    }

    pub fn run(&mut self) {
        println!(
            "RT-NDS server running on {}",
            self.socket.local_addr().unwrap()
        );
        let mut buf = [0; PACKET_SIZE];
        loop {
            let (amt, src) = self.socket.recv_from(&mut buf).unwrap();
            let packet = Packet::from_buffer(&buf[..amt]);
            if packet.token == TOKEN {
                if packet.ptype == PacketType::Set as u8 {
                    println!("Set {} as {:?}", packet.data_name, packet.data);
                    self.cache
                        .set(&packet.data_name, packet.data, Duration::from_secs(60));
                }
                if packet.ptype == PacketType::Get as u8 {
                    println!("Get {}", packet.data_name);
                    let data = self.cache.get(&packet.data_name);
                    if data.is_none() {
                        continue;
                    }
                    let mut response_packet = Packet::default();
                    response_packet.data = data.unwrap().clone();
                    let _ = self.socket.send_to(&response_packet.to_buffer(), src);
                }
            }
        }
    }
}
