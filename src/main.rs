use std::net::{TcpStream, Shutdown, Ipv4Addr};
use std::io::{Write, Read};
use std::time::{Duration};
use std::env;

mod varint;
mod packet;
mod config;

fn main() {
    let mut config = config::Config::new(env::args()).unwrap();
    config.evaluate();

    if config.ipv4 == None {println!("Failed to get starting IP"); }
    else {println!("Start IP: {}", config.ipv4.clone().unwrap())}

    if config.end_ipv4 == None {println!("Failed to get starting IP"); }
    else {println!("End IP: {}", config.end_ipv4.clone().unwrap()); }

    for i in 0..4 {
        println!("Diff arr: {}", config.diff_arr.clone().unwrap()[i]);
    }

    //Just separating this for now so I can continue testing CLI stuff without actually pinging
    
    if false {
        let port: u16 = 25565;
        let address: String = config.ipv4.clone().expect("Must have IP address");
        let mut tcp_stream = TcpStream::connect(format!("{}:{}", address, port)).unwrap();

        //timeout configuration
        tcp_stream.set_read_timeout(Some(Duration::new(5, 0))).expect("Failed to set read timeout duration");
        tcp_stream.set_write_timeout(Some(Duration::new(5, 0))).expect("Failed to set write timeout duration");
        tcp_stream.set_ttl(100).expect("Failed to set write TTL duration");

        //handshake packet
        let hs_packet = packet::Packet::handshake_packet(774, address.clone(), port.clone());
        tcp_stream.write_all(&hs_packet.data).unwrap();

        //status request packet
        let mut sr_packet: Vec<u8> = Vec::new();
        sr_packet.push(0x01); // 1 byte
        sr_packet.push(0x00); // status request byte.
        tcp_stream.write_all(&sr_packet).unwrap();

        //reading client bound stuff
        let tuple = varint::tcp_read_var_int(&mut tcp_stream).try_into().unwrap();
        let (size, byte_num) = tuple;

        let mut string_data: Vec<u8> = vec![0u8; size as usize];
        tcp_stream.read_exact(&mut string_data).unwrap();
        string_data.drain(0..byte_num);
        
        let print_string = config.byte_to_utf8_conv(&string_data);
        println!("Print string: {}", print_string);

        tcp_stream.shutdown(Shutdown::Both).unwrap();
    }
}
