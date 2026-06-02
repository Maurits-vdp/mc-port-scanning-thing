use crate::varint::{VarInt, write_var_int};

// Pushes varint, ignoring empty bytes.
pub fn packet_push_var_int(packet: &mut Vec<u8>, var: VarInt){
    let mut i: usize = 0;
    while i < var.siz {
        packet.push(var.v[i]);
        i += 1;
    }
}

pub struct Packet{
    pub data: Vec<u8>,
}

impl Packet {
    pub fn handshake_packet(proto_v: i32, addr_ipv4: String, port: u16) -> Self {
        let mut data_buffer: Vec<u8> = Vec::new();

        packet_push_var_int(&mut data_buffer, write_var_int(0x00));
        packet_push_var_int(&mut data_buffer, write_var_int(proto_v)); // Protocol version 
        packet_push_var_int(&mut data_buffer, write_var_int(addr_ipv4.len() as i32)); // ipv4 address lenght
        data_buffer.extend(addr_ipv4.as_bytes()); // ipv4 address
        data_buffer.extend(port.to_be_bytes()); //port number
        data_buffer.push(0x01); //next state var int

        let len: VarInt = write_var_int(data_buffer.len() as i32);
        data_buffer.splice(0..0, len.v[..len.siz].into_iter().cloned());

        Self{data: data_buffer} 
    }
}
