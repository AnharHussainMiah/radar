use std::collections::HashMap;
use std::path::Path;
use std::{str, usize};

/* -------------------------------------------------------------------------------------------------
| I've shamelessly stoten this from "https://github.com/fristonio/docker.rs" and modified it,
| who originally ripped it from "https://github.com/p00s/minihttpse" (that now doesn't exist?!)
--------------------------------------------------------------------------------------------------*/

const CR: u8 = b'\r';
const LF: u8 = b'\n';

#[derive(Debug)]
pub struct Response {
    pub status_code: usize,
    pub body: String,
}

impl Response {
    pub fn parse_http_response(res: Vec<u8>) -> Result<Response, String> {
        let mut pos: usize = 0;
        for i in 0..(res.len() - 1) {
            if res[i] == CR && res[i + 1] == LF && res[i + 2] == CR && res[i + 3] == LF {
                pos = i + 3;
                break;
            }
        }

        if pos == 0 {
            return Err("Not a valid HTTP response".to_string());
        }

        let (resp_header, resp_body): (&[u8], &[u8]) = res.split_at(pos);

        let header_info = match String::from_utf8(resp_header.to_vec()) {
            Ok(h) => h,
            Err(_) => return Err("Error while parsing HTTP header".to_string()),
        };

        let body = resp_body[1..].to_owned();

        let mut header_vec: Vec<&str> = header_info.split("\r\n").collect();
        let status = header_vec[0].to_owned();
        let status_vec: Vec<&str> = status.splitn(3, " ").collect();

        let status_code: usize = match status_vec[1].parse() {
            Ok(s) => s,
            Err(_) => return Err("Error while parsing HTTP status code".to_string()),
        };

        header_vec.remove(0);
        let len = header_vec.len();
        header_vec.remove(len - 1);

        let mut headers: HashMap<String, String> = HashMap::new();
        for header in header_vec {
            let item = header.to_owned();
            let item_vec: Vec<&str> = item.splitn(2, ": ").collect();
            headers.insert(item_vec[0].to_owned(), item_vec[1].to_owned());
        }

        let body = match headers.get("Transfer-Encoding") {
            Some(enc) => {
                if enc == "chunked" {
                    Response::parse_chunk(body)?
                } else {
                    body
                }
            }
            None => body,
        };

        let response = match String::from_utf8(body) {
            Ok(s) => s.trim().to_owned(),
            Err(_) => return Err("Error while parsing response body".to_string()),
        };

        Ok(Response {
            status_code: status_code,
            body: response,
        })
    }

    /// A helper function to parse_http_reseponse, when the Header Transfer-Encoding
    /// `chunked` is present in the response.
    pub fn parse_chunk(body: Vec<u8>) -> Result<Vec<u8>, String> {
        let mut buf: Vec<u8> = Vec::new();
        let mut count: usize = 0;

        loop {
            let mut pos: usize = 0;
            for i in count..body.len() - 1 {
                if body[i] == CR && body[i + 1] == LF {
                    pos = i;
                    break;
                }
            }
            if pos == 0 {
                return Err("Chuncked response without length marker".to_string());
            }

            let size_s = match str::from_utf8(&body[count..pos]) {
                Ok(s) => s,
                Err(_) => return Err("Invlid chunks".to_string()),
            };

            count = pos + 2;
            let size: usize = match usize::from_str_radix(size_s, 16) {
                Ok(s) => s,
                Err(_) => return Err("Invalid chunks".to_string()),
            };

            if size == 0 && count + 2 == body.len() {
                return Ok(buf);
            }

            buf.extend_from_slice(&body[pos + 2..pos + 2 + size]);
            count = count + size + 2;
        }
    }
}
