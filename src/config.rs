use crate::Ipv4Addr;
use std::process::exit;
use std::time::Duration;

pub struct Config {
    pub ipv4: Option<[u8; 4]>,
    pub end_ipv4: Option<[u8; 4]>,
    pub delay: Duration,

    pub diff_arr: Option<[u8; 4]>,

    //flags
    pub help: Option<()>,
    pub range: Option<()>,
    pub force: Option<()>,
}

fn convert_ipstr_u8_arr(string: &str) -> [u8; 4]{
    let s_vec: Vec<&str> = string.split('.').collect();
    return [s_vec[0].parse::<u8>().unwrap(), s_vec[1].parse::<u8>().unwrap(), s_vec[2].parse::<u8>().unwrap(), s_vec[3].parse::<u8>().unwrap()];
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        /* CLI input guide structure stuff:
         * If we have -r set we expect input: exec -r start_ip end_ip ... 
         * If we have -h set we expect input: exec -h
         * If we have neither -r or -h set we expect input: exec ip_address ...
         * */

        args.next();
        let mut ip: Option<[u8; 4]> = None;
        let mut end_ip: Option<[u8; 4]> = None;
        let mut delay: Duration = Duration::from_millis(100);

        //flags
        let mut help_flg: Option<()> = None;
        let mut range_flg: Option<()> = None;
        let mut force_flg: Option<()> = None;

        while let Some(arg) = args.next() {
            let valid_ip = arg.parse::<Ipv4Addr>().is_ok();

            if valid_ip & !ip.is_some() {
                ip = Some(convert_ipstr_u8_arr(&arg));
                continue;
            } else if valid_ip & ip.is_some() & range_flg.is_some(){
                end_ip = Some(convert_ipstr_u8_arr(&arg));
                continue;
            }

            match arg.as_str() {
                "-h" => help_flg = Some(()), // Help flag
                "-r" => range_flg = Some(()), // Use IP Range flag: -r start_ip
                "-F" => force_flg = Some(()), // Force flag for forced client bound type conversion
                "-d" => delay = {
                    let var = args.next().unwrap(); 
                    println!("Setting delay: {}", var); 
                    Duration::from_millis(var.parse::<u64>().unwrap())
                },
                _ => continue,
            }
        }

        Ok( Config {
            ipv4: ip,
            end_ipv4: end_ip,

            diff_arr: None,
            delay: delay,

            help: help_flg,
            range: range_flg,
            force: force_flg,
        })
    }
    pub fn evaluate<'a>(&'a mut self){
        self.handle_help_flag();
        self.handle_range_flag();
        self.handle_force_flag();
    }

    fn handle_help_flag<'a>(&'a self){
        match self.help {
            None => {return; },
            Some(()) => {
                println!("mc_port_scanner help:
                    [path to executable] [ip to scan]: Scan the provided IP on port 25565
                    -h: help
                    -r [start ip] [end_ip]: Specify IP range (-r start_ip end_ip): (e.g. -r 192.168.0.0 192.168.3.23)
                    -F: Force UTF8 type conversion. (This forces rust to convert byte to string in an unsafe block)
                    -d [delay]: Set a time in ms for the program to sleep before sending packets to the next IP. The default is 100 ms"); 
                exit(0);
            },
        }
    }

    //Creates a differences in IP range array and assigns it.
    //This is used to iterate over multiple IP addresses later
    fn handle_range_flag<'a>(&'a mut self){
        match self.range {
            None => {
                self.diff_arr = Some([0u8; 4]);
            }
            Some(()) => {
                let start_ip = self.ipv4.clone().unwrap(); 
                let end_ip = self.end_ipv4.clone().unwrap();

                let mut diff_arr = [0u8; 4];
                for i in 0..4 {
                   diff_arr[i] = end_ip[i] - start_ip[i]; 
                }
                self.diff_arr = Some(diff_arr);
            }
        }
    }
    fn handle_force_flag<'a>(&'a self){
        match self.force {
            None => return,
            Some(()) => println!("Warning: using forceful conversion of client bound bytes to UTF8 string necessitates the use of an unsafe type conversion!"),
        }
    }
    //NOTE perhaps use a closure or something so I don't have to constantly use match
    pub fn byte_to_utf8_conv<'a>(&'a self, data: &'a Vec<u8>) -> &'a str {
        match self.force {
            None => {
                str::from_utf8(data).unwrap()
            }
            Some(()) => {
                unsafe {
                    str::from_utf8_unchecked(data)
                }
            }
        }
    }
}
