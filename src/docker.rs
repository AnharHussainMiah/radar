use crate::http::Response;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct Docker {
    socket: UnixStream,
}

pub enum RequestVerb {
    GET,
    POST,
}

impl Docker {
    pub fn new() -> Result<Docker, String> {
        // trying connecting to the unix docker.socket
        let raw_socket = match UnixStream::connect("/var/run/docker.sock") {
            Ok(sock) => sock,
            Err(err) => return Err(err.to_string()),
        };

        Ok(Docker { socket: raw_socket })
    }

    fn dial(&mut self, request: &str) -> Option<String> {
        match self.socket.write_all(request.as_bytes()) {
            Ok(_) => println!("==> bytes written to unix socket"),
            Err(err) => println!("==> ERROR: unable to write to unix socket: {}", err),
        }

        let mut buffer: [u8; 1024] = [0; 1024];
        let mut raw: Vec<u8> = Vec::new();
        loop {
            let len = match self.socket.read(&mut buffer) {
                Ok(len) => len,
                Err(err) => {
                    println!("{}", err);
                    return None;
                }
            };
            for i in 0..len {
                raw.push(buffer[i]);
            }
            if len < 1024 {
                break;
            }
        }
        Some(String::from_utf8_lossy(&raw).to_string())
    }

    fn build_request(verb: RequestVerb, url: &str, payload: Option<String>) -> String {
        match verb {
            RequestVerb::GET => {
                format!("GET {url} HTTP/1.1\r\nHost: v1.37\r\n\n", url = url)
            }
            RequestVerb::POST => {
                format!(
                    "POST {url} HTTP/1.1\r\nHost: v1.37\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{payload}\r\n\r\n"
                    ,url = url
                    ,length = payload.as_ref().unwrap().len()
                    ,payload = payload.unwrap()
                )
            }
        }
    }

    pub fn get(&mut self, url: &str) -> Option<String> {
        let request = Docker::build_request(RequestVerb::GET, url, None);

        if let Some(response) = Docker::dial(self, &request) {
            return match Response::parse_http_response(response.into()) {
                Ok(parsed) => Some(parsed.body),
                Err(e) => {
                    println!("ERROR: {}", e);
                    None
                }
            };
        }
        return None;
    }

    pub fn post(&mut self, url: &str, payload: &str) -> Option<String> {
        let request = Docker::build_request(RequestVerb::POST, url, Some(payload.to_string()));

        if let Some(response) = Docker::dial(self, &request) {
            return match Response::parse_http_response(response.into()) {
                Ok(parsed) => Some(parsed.body),
                Err(e) => {
                    println!("ERROR: {}", e);
                    None
                }
            };
        }
        return None;
    }

    pub fn list_images(&mut self) -> Vec<String> {
        if let Some(data) = Docker::get(self, "/images/json") {
            let json: Vec<HashMap<String, serde_json::Value>> =
                serde_json::from_str(&data).expect("unable to parse json");
            let images = json
                .iter()
                .map(|x| x["Id"].to_string().replace("\"", ""))
                .collect();
            return images;
        }
        Vec::new()
    }

    pub fn list_containers(&mut self) -> Vec<String> {
        if let Some(data) = Docker::get(self, "/containers/json") {
            let json: Vec<HashMap<String, serde_json::Value>> =
                serde_json::from_str(&data).expect("unable to parse json");
            // println!("{:?}", json);
            let images = json
                .iter()
                .map(|x| x["Id"].to_string().replace("\"", ""))
                .collect();
            return images;
        }
        Vec::new()
    }

    pub fn inspect(&mut self, container_id: String) -> Option<String> {
        if let Some(data) = Docker::get(self, &format!("/containers/{}/json", container_id)) {
            // println!("INSPECT -> {}", data);
            return Some(data.to_string());
        }
        None
    }
}
