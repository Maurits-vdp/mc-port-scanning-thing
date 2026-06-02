use crate::TcpStream;
use crate::Read;

pub struct VarInt{
    pub v: [u8; 5],
    pub siz: usize, //Some may not use the whole array for storing a VarInt, this is here to help with
                //that
}

//read from tcp input stream to get VarInt prefix. Used to find status string lenght
pub fn tcp_read_var_int(tcp_stream: &mut TcpStream) -> (i32, usize) {
    let mut value: i32 = 0;
    let mut shift: usize = 0;
    let mut byte_num: usize = 0;
    //byte_num is for how many bytes are from the varint

    loop {
        byte_num += 1;
        let mut buf = [0u8; 1];
        tcp_stream.read_exact(&mut buf).unwrap();
        let current = buf[0];
        value |= ((current & 0x7F) as i32 ) << shift;
        
        if (current & 0x80) == 0 { break; }

        shift += 7;

        if shift >= 32 { panic!("Too large for var int")};
    }

    return (value, byte_num);
}

pub fn write_var_int(value: i32) -> VarInt {
    let mut vout = VarInt{
        v: [0; 5],
        siz: 0, //start at index 0, then use more bytes if needed.
    };
    let mut vcopy = value;

    let mut v_byte;
    loop {
        v_byte = (vcopy & 0x7F) as u8;
        vcopy >>= 7;
        if vcopy != 0 {
            v_byte |= 0x80; 
        }
        vout.v[vout.siz] = v_byte;
        if vcopy == 0 { break; }
        vout.siz += 1;
    }
    return vout;
}
