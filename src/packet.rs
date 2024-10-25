pub enum PacketType {
    Set = 1,
    Get = 2,
}
pub const PACKET_SIZE: usize = 32864;
pub const MAX_DATA_SIZE: usize = 32768;
pub const TOKEN: u64 = 0;

pub struct Packet {
    pub magic: u16,
    pub uid: u64,
    pub ptype: u8,
    pub token: u64,
    pub priority: u8,
    pub data_name: String, // max 64 bytes
    pub data_size: u32,
    pub data_chunk_num: u16,
    pub data_chunk_id: u16,
    pub data_checksum: u32,
    pub data_expiry: u64, // expire in ? seconds
    pub data: Vec<u8>,    // max 32768 bytes
}

impl Packet {
    pub fn default() -> Self {
        Self {
            magic: 0xCAFE,
            uid: 1,
            ptype: 0,
            token: TOKEN,
            priority: 0,
            data_name: String::new(),
            data_size: 0,
            data_chunk_num: 0,
            data_chunk_id: 0,
            data_checksum: 0,
            data_expiry: 10,
            data: Vec::new(),
        }
    }

    pub fn from_buffer(buffer: &[u8]) -> Self {
        let mut packet = Self::default();
        packet.magic = u16::from_be_bytes(buffer[0..2].try_into().unwrap());
        packet.uid = u64::from_be_bytes(buffer[2..10].try_into().unwrap());
        packet.ptype = buffer[10];
        packet.token = u64::from_be_bytes(buffer[11..19].try_into().unwrap());
        packet.priority = buffer[19];
        packet.data_name = String::from_utf8_lossy(&buffer[20..84]).to_string();
        packet.data_size = u32::from_be_bytes(buffer[84..88].try_into().unwrap());
        packet.data_chunk_num = u16::from_be_bytes(buffer[88..90].try_into().unwrap());
        packet.data_chunk_id = u16::from_be_bytes(buffer[90..92].try_into().unwrap());
        packet.data_checksum = u32::from_be_bytes(buffer[92..96].try_into().unwrap());
        packet.data_expiry = u64::from_be_bytes(buffer[96..104].try_into().unwrap());
        packet.data = buffer[104..].to_vec();
        packet
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.magic.to_be_bytes());
        buffer.extend_from_slice(&self.uid.to_be_bytes());
        buffer.push(self.ptype);
        buffer.extend_from_slice(&self.token.to_be_bytes());
        buffer.push(self.priority);
        let mut data_name_bytes = self.data_name.as_bytes().to_vec();
        data_name_bytes.resize(64, 0);
        buffer.extend_from_slice(&data_name_bytes);
        buffer.extend_from_slice(&self.data_size.to_be_bytes());
        buffer.extend_from_slice(&self.data_chunk_num.to_be_bytes());
        buffer.extend_from_slice(&self.data_chunk_id.to_be_bytes());
        buffer.extend_from_slice(&self.data_checksum.to_be_bytes());
        buffer.extend_from_slice(&self.data_expiry.to_be_bytes());
        buffer.extend_from_slice(&self.data);
        buffer
    }
}
