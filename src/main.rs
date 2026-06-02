use std::net::{TcpStream, Shutdown, Ipv4Addr};
use std::io::{Write, Read};
use std::time::{Duration};
use std::str;
use std::env;

mod varint;

fn packet_push_var_int(packet: &mut Vec<u8>, var: varint::VarInt){
    let mut i: usize = 0;
    while i <= var.siz {
        packet.push(var.v[i]);
        i += 1;
    }
}

struct Config {
    ipv4: Option<String>,
    end_ipv4: Option<String>,
    
    //flags
    help: Option<bool>,
    range: Option<bool>,
}

impl Config {
    fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        /* CLI input guide structure stuff:
         * If we have -r set we expect input: exec -r start_ip end_ip ... 
         * If we have -h set we expect input: exec -h
         * If we have neither -r or -h set we expect input: exec ip_address ...
         * */

        args.next();
        let mut ip: Option<String> = None;
        let mut end_ip: Option<String> = None;

        //flags
        let mut h: Option<bool> = None;
        let mut r: Option<bool> = None;

        for arg in args {
            let valid_ip = arg.parse::<Ipv4Addr>().is_ok();

            if valid_ip & !ip.is_some() {
                ip = Some(arg.clone());
                continue;
            } else if valid_ip & ip.is_some() {
                end_ip = Some(arg.clone());
                continue;
            }

            match arg.as_str() {
                "-h" => h = Some(true), // Help flag
                "-r" => r = Some(true), // Use IP Range flag: -r start_ip
                _ => continue,
            }
        }

        Ok( Config {
            ipv4: ip,
            end_ipv4: end_ip,

            help: h,
            range: r,
        })
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap();

    let port: u16 = 25565;
    let addres: String = config.ipv4.expect("Must have IP address").clone();
    let mut tcp_stream = TcpStream::connect(format!("{}:{}", addres, port)).unwrap();

    //timeout configuration
    tcp_stream.set_read_timeout(Some(Duration::new(5, 0))).expect("Failed to set read timeout duration");
    tcp_stream.set_write_timeout(Some(Duration::new(5, 0))).expect("Failed to set write timeout duration");
    tcp_stream.set_ttl(100).expect("Failed to set write TTL duration");

    //buffer packet
    println!("Creating buffer packet");
    let mut buf_packet: Vec<u8> = Vec::new(); 
    packet_push_var_int(&mut buf_packet, varint::write_var_int(0x00)); //packet ID
    packet_push_var_int(&mut buf_packet, varint::write_var_int(774)); //protocol version
    packet_push_var_int(&mut buf_packet, varint::write_var_int(addres.len() as i32)); //prefix address length
    buf_packet.extend(addres.as_bytes()); //address
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
