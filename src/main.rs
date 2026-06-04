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

    let diff_arr = config.diff_arr.clone().unwrap();
    for i in 0..4 {
        println!("diff array at {}: {}", i, diff_arr[i]);
    }

    let port: u16 = 25565;

    //status request packet
    let mut sr_packet: Vec<u8> = Vec::new();
    sr_packet.push(0x01); // 1 byte
    sr_packet.push(0x00); // status request byte.

    let mut address: [u8; 4];

    /*The order here of the address array is reversed for a bit, this is important
     * Example: diff_arr = 127.0.0.6 - 127.0.0.1 <= this implies that 127 is of order 256^3 and
     * because of the way we handle stuff 127 ends up at index zero
     */
    let iter_max: u32 = diff_arr[3] as u32 + diff_arr[2] as u32 * 256 + diff_arr[1] as u32 * 256 * 256 + diff_arr[0] as u32 * 256 * 256 * 256;

    println!("Start address: {}.{}.{}.{}", start_addr[0], start_addr[1], start_addr[2], start_addr[3]);
    let start_addr_as_u32 = u32::from_be_bytes(start_addr);

    for i in 0..iter_max{
        address = (start_addr_as_u32 + i).to_be_bytes(); 
        println!("Attempting to handshake: {}.{}.{}.{}", address[0], address[1], address[2], address[3]);
        
        //NOTE this is set to false so I don't accidentally start spamming a bunch of ip addresses
        //while I am testing stuff. In a proper setting this should be set to true 
        if false {
            let socket = SocketAddrV4::new(Ipv4Addr::new(address[0], address[1], address[2], address[3]), port);
            let mut tcp_stream = TcpStream::connect(socket).unwrap();

            //timeout configuration
            tcp_stream.set_read_timeout(Some(Duration::new(5, 0))).expect("Failed to set read timeout duration");
            tcp_stream.set_write_timeout(Some(Duration::new(5, 0))).expect("Failed to set write timeout duration");
            tcp_stream.set_ttl(100).expect("Failed to set write TTL duration");

            //handshake packet
            let hs_packet = packet::Packet::handshake_packet(774, address, port);

            // Sending packets
            tcp_stream.write_all(&hs_packet.data).unwrap();
            tcp_stream.write_all(&sr_packet).unwrap();

            // Reading client bound stuff
            let tuple = varint::tcp_read_var_int(&mut tcp_stream).try_into().unwrap();
            let (size, byte_num) = tuple;

            let mut string_data: Vec<u8> = vec![0u8; size as usize];

            // Draining extra bytes at start
            tcp_stream.read_exact(&mut string_data).unwrap();
            string_data.drain(0..byte_num);

            let print_string = config.byte_to_utf8_conv(&string_data);
            println!("Print string: {}\n", print_string);

            tcp_stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}
