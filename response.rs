use headers::Headers;
mod headers;

#[deriving(ToStr)]
pub struct Response {
    headers: Headers,
    body: ~[u8]
}

impl Response {
    pub fn new(headers: Headers, body: ~[u8]) -> Response {
        Response {headers: headers, body: body}
    }
}
