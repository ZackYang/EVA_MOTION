extern crate bitty;
use bitty::{AsBits, FromBits};
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = vec![0u8;17];
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            if size > 0 {
                println!("{:?}", data.to_vec());
            }
            let mut sending_stream = stream.try_clone().unwrap();
            thread::spawn(move|| { 
                // for i in 0u8..255u8 {
                    let new_data = vec![
                        false, false, false, false, false, true, true, true,

                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,

                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true, 
                        
                        false, false, false, false, false, true, false, true,

                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true, 
                        
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true,
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true, 
                        false, false, false, false, false, false, false, true];
                    
                    let mut u8_data = vec![0u8; 0];

                    for chunk in new_data.to_vec().chunks(8) {
                        let mut _bits = chunk.to_vec();
                        _bits.reverse();
                        u8_data.push(u8::from_bits(&_bits[..]));
                    };

                    std::thread::sleep_ms(5000);
                    sending_stream.write(&u8_data).unwrap();
                // }
            });

            true
        },
        Err(_) => {
            // println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}