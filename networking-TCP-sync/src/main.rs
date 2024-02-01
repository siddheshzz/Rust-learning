use std::io::{Read, Write};
use std::net::{TcpListener,TcpStream};


fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0 as u8;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {:?}",String::from_utf8_lossy(&buffer[..]));
}

fn main() {
    println!("Welcome to Synchronous TCP");
    let server = TcpListener::bind("127.0.0.1:80").expect("Could not Start the server");
    println!("Server listening on 127.0.0.1:80");

    for stream in server.incoming(){
        
        match stream{
            
            Ok(stream) => {
                /*below prints TcpStream { addr: 127.0.0.1:80, peer: 127.0.0.1:53539, socket: 232 }
                which is printing stream but we need to READ THE STREAM*/

                // println!("{:?}",stream);
                handle_connection(stream);
            }
            Err(e) => { /* connection failed */
            println!("Establishing connection failed {}",e) }
        }
    
    }


    



}
