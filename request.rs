use headers::Headers;
mod headers;

pub enum Method {
    GET,
    POST
}

#[deriving(ToStr)]
pub struct Request {
    url: ~str,
    headers: Headers,
    body: ~[u8]
}

impl Request {
    
    pub fn new(url: ~str, headers: Headers, body: ~[u8]) -> Request {
        Request {url: url, headers: headers, body: body}
    }
}

