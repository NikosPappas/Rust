use std::net::TcpStream;
use std::str;
use std::io::{self,BufRead,BufReader,Write};

fn main() {
   let mut stream=TcpStream::connect("127.0.0.1:8888")
                             .expect("Could not connect to server");
   loop{
      let mut _input:String=String::new();
      let mut _buffer:Vec<u8>=Vec::new();
      io::stdin().read_line(&mut _input).expect("Failed to read from stdin");
      stream.write(_input.as_bytes()).expect("Failed to write to server");
      let mut _reader = BufReader::new(&stream);
      _reader.read_until(b'\n',&mut _buffer).expect("Could not read into buffer");
      print!("{:?}",str::from_utf8(&_buffer).expect("Could not write buffer as string"));
   }//end loop
}
