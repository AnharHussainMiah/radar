
use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct Docker {
    socket: UnixStream
}

impl Docker {
    pub fn new() -> Result<Docker, String> {
        // trying connecting to the unix docker.socket
        let raw_socket = match UnixStream::connect("/var/run/docker.sock") {
            Ok(sock) => sock,
            Err(err) => {
                return Err(err.to_string())
            } 
        };

        Ok(Docker {
            socket: raw_socket
        })
    }

    pub fn call(mut self, request: &str) -> Option<String> {
        match self.socket.write_all(request.as_bytes()) {
            Ok(_) => println!("==> bytes written to unix socket"),
            Err(err) => println!("==> ERROR: unable to write to unix socket: {}", err)
        }

        let mut buffer: [u8; 1024] = [0; 1024];
        let mut raw: Vec<u8> = Vec::new();
        loop {
            let len = match self.socket.read(&mut buffer) {
                Ok(len) => len,
                Err(_) => return None,
            };

            for i in 0..len { raw.push(buffer[i]); }
            if len < 1024 { break; }
        }
        Some(String::from_utf8_lossy(&raw).to_string())
    }
}