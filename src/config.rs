use crate::Ipv4Addr;
use std::process::exit;

pub struct Config {
    pub ipv4: Option<String>,
    pub end_ipv4: Option<String>,

    pub diff_arr: Option<[u8; 4]>,

    //flags
    pub help: Option<()>,
    pub range: Option<()>,
    pub force: Option<()>,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        /* CLI input guide structure stuff:
         * If we have -r set we expect input: exec -r start_ip end_ip ... 
         * If we have -h set we expect input: exec -h
         * If we have neither -r or -h set we expect input: exec ip_address ...
         * */

        args.next();
        let mut ip: Option<String> = None;
        let mut end_ip: Option<String> = None;

        //flags
        let mut help_flg: Option<()> = None;
        let mut range_flg: Option<()> = None;
        let mut force_flg: Option<()> = None;

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
                "-h" => help_flg = Some(()), // Help flag
                "-r" => range_flg = Some(()), // Use IP Range flag: -r start_ip
                "-F" => force_flg = Some(()), // Force flag for forced client bound type conversion
                _ => continue,
            }
        }

        Ok( Config {
            ipv4: ip,
            end_ipv4: end_ip,

            diff_arr: None,

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
                println!("mc_port_scanner help:\n
                    scan IP: executable ip_to_scan\n
                    -h: help\n
                    -r: specify IP range (-r start_ip end_ip): (e.g. -r 192.168.0.0 192.168.3.23)\n
                    -F: Force UTF8 type conversion. (This forces rust to convert byte to string in an unsafe block)"); 
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
                let start_ip = self.ipv4.clone().expect("Unable to parse string for asserting length!");
                let end_ip = self.end_ipv4.clone().expect("Unable to parse string for asserting length!");

                assert_eq!(start_ip.matches(".").collect::<Vec<&str>>().len(), 3);
                assert_eq!(end_ip.matches(".").collect::<Vec<&str>>().len(), 3);

                //parsing addresses to u8 arrays for better numerical handling
                let s_vec: Vec<&str> = start_ip.split('.').collect();
                let f_vec: Vec<&str> = end_ip.split('.').collect();

                let mut diff_arr = [0u8; 4];
                for i in 0..4 { 
                    let value: u8 = f_vec[i].parse::<u8>().unwrap() - s_vec[i].parse::<u8>().unwrap();
                    diff_arr[i] = value;
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
