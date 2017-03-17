use std::net::TcpListener;
use std::net::TcpStream;
use std::string::String;
use std::io::Read;
use std::io::Write;

pub struct Network {
    listener: TcpListener,
}

impl Network {
    pub fn create() -> Network {
        let listener = TcpListener::bind("127.0.0.1:4551").unwrap();
        Network { listener: listener }
    }

    pub fn listen(&self) {
        trace!("Listening to network");
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut request = String::new();
                    stream.read_to_string(&mut request).ok();
                    stream.write_all(b"my test string repley");
                    println!("works, {}", request);
                }
                Err(e) => { println!("error") }
            }
        }
    }

    pub fn close(&self) {}
}