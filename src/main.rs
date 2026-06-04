use std::net::{TcpStream, Shutdown, Ipv4Addr, SocketAddrV4};
use std::io::{Write, Read};
use std::time::{Duration};
use std::env;

mod varint;
mod packet;
mod config;

fn main() {
    let mut config = config::Config::new(env::args()).unwrap();
    config.evaluate();

    let start_addr: [u8; 4] = match config.ipv4 {
        None => {println!("Failed to get starting IP"); panic!("Failed to get starting IP");},
        Some(_) => config.ipv4.unwrap(),
    };

    //NOTE Add support for range flag so it actually does stuff
    let end_addr: [u8; 4] = match config.end_ipv4 {
        None => config.ipv4.unwrap(),
        Some(_) => config.end_ipv4.unwrap(),
    };

    let diff_arr = config.diff_arr.clone().unwrap();
    for i in 0..4 {
        println!("diff array at {}: {}", i, diff_arr[i]);
    }

    //Just separating this for now so I can continue testing CLI stuff without actually pinging
    if false {
        let port: u16 = 25565;
        let socket = SocketAddrV4::new(Ipv4Addr::new(start_addr[0], start_addr[1], start_addr[2], start_addr[3]), port);
        let mut tcp_stream = TcpStream::connect(socket).unwrap();

        //timeout configuration
        tcp_stream.set_read_timeout(Some(Duration::new(5, 0))).expect("Failed to set read timeout duration");
        tcp_stream.set_write_timeout(Some(Duration::new(5, 0))).expect("Failed to set write timeout duration");
        tcp_stream.set_ttl(100).expect("Failed to set write TTL duration");

        //handshake packet
        let hs_packet = packet::Packet::handshake_packet(774, start_addr.clone(), port.clone());
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
