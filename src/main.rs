use std::net::{TcpStream, Shutdown};
use std::io::{Write, Read};
use std::time::{Duration};
use std::str;

mod varint;

fn packet_push_var_int(packet: &mut Vec<u8>, var: varint::VarInt){
    let mut i: usize = 0;
    while i <= var.siz {
        packet.push(var.v[i]);
        i += 1;
    }
}

fn main() {
    let ipv4address = "ipaddress"; //put IP address here. Will setup CLI later
    let port: u16 = 25565;
    let mut tcp_stream = TcpStream::connect(format!("{}:{}", ipv4address, port)).unwrap();

    //timeout configuration
    tcp_stream.set_read_timeout(Some(Duration::new(5, 0))).expect("Failed to set read timeout duration");
    tcp_stream.set_write_timeout(Some(Duration::new(5, 0))).expect("Failed to set write timeout duration");
    tcp_stream.set_ttl(100).expect("Failed to set write TTL duration");

    //buffer packet
    println!("Creating buffer packet");
    let mut buf_packet: Vec<u8> = Vec::new(); 
    packet_push_var_int(&mut buf_packet, varint::write_var_int(0x00)); //packet ID
    packet_push_var_int(&mut buf_packet, varint::write_var_int(774)); //protocol version
    packet_push_var_int(&mut buf_packet, varint::write_var_int(ipv4address.len() as i32)); //prefix address length
    buf_packet.extend(ipv4address.as_bytes()); //address
    buf_packet.extend(port.to_be_bytes()); //port number
    buf_packet.push(0x01); //next state var int

    let mut hs_packet: Vec<u8> = Vec::new();
    packet_push_var_int(&mut hs_packet, varint::write_var_int(buf_packet.len() as i32));
    hs_packet.extend_from_slice(&buf_packet);
    tcp_stream.write_all(&hs_packet).unwrap();

    //status request packet
    println!("Creating status request packet");
    let mut sr_packet: Vec<u8> = Vec::new();
    sr_packet.push(0x01); // 1 byte
    sr_packet.push(0x00); // status request byte.
    tcp_stream.write_all(&sr_packet).unwrap();

    let tuple = varint::tcp_read_var_int(&mut tcp_stream).try_into().unwrap();
    let (size, byte_num) = tuple;

    println!("Packet size: {}", size);

    let mut string_data: Vec<u8> = vec![0u8; size as usize];
    tcp_stream.read_exact(&mut string_data).unwrap();
    string_data.drain(0..byte_num);
    for byte in string_data.clone() {
        //this is a debug print I'm not fixing right now because it's nearly 3 in the morning
        println!("Byte: {}", byte);
    }
    
    let print_string = str::from_utf8(&string_data).unwrap();
    println!("Print string:\n{}", print_string);

    tcp_stream.shutdown(Shutdown::Both).unwrap();
}
