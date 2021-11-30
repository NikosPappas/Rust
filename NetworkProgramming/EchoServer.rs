use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

fn main(){
  let listener = TcpListener::bind("127.0.0.1:888")
                             .expect("Could not bind");
  for stream in listener.incoming(){
        match stream{
           Err(e) => { eprintln!("Failed: {}",e)}
           Ok(stream)=>{
                    thread::spawn(move ||{
                          handle_client(stream)
                          .unwrap_or_else(|error| eprintln!("{:?}",error));
                    });
          }//end of Ok clause
       }//end match clause
  }//end for loop
}
fn handle_client(mut stream:TcpStream)->Result<(),Error>{
          println!("Incoming connection from: {}",stream.peer_addr()?);
          let mut buf=[0;512];
          loop{
              let bytes_read = stream.read(&mut buf)?;
              if bytes_read==0 { return Ok(());}
              stream.write(&buf[..bytes_read])?;
          }//end of loop action
}
