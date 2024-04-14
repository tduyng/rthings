use bytes::BytesMut;
use std::{fmt, io, str};

pub struct Request {
    method: String,
    path: String,
    version: u8,
    headers: Vec<(String, Vec<u8>)>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: String::new(),
            path: String::new(),
            version: 0,
            headers: Vec::new(),
        }
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn headers(&self) -> &[(String, Vec<u8>)] {
        &self.headers
    }

    pub fn decode(mut buf: BytesMut) -> io::Result<Option<Self>> {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut r = httparse::Request::new(&mut headers);
        let status = r.parse(&buf).map_err(|e| {
            let msg = format!("failed to parse http request: {:?}", e);
            io::Error::new(io::ErrorKind::Other, msg)
        })?;

        match status {
            httparse::Status::Complete(amt) => {
                let method = r.method.unwrap().to_string();
                let path = r.path.unwrap().to_string();
                let version = r.version.unwrap();

                let parsed_headers: Vec<(String, Vec<u8>)> = r
                    .headers
                    .iter()
                    .map(|h| (h.name.to_string(), h.value.to_vec()))
                    .collect();

                let _ = buf.split_to(amt);

                Ok(Some(Request {
                    method,
                    path,
                    version,
                    headers: parsed_headers,
                }))
            }
            httparse::Status::Partial => Ok(None),
        }
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<HTTP Request {} {}", self.method, self.path)
    }
}
