use headers::Headers;

/// Represents HTTP request methods
pub enum Method {
    GET,
    POST
}

/// The HTTP request struct
#[deriving(Show)]
pub struct Request {
    url: ~str,
    headers: Headers,
    body: ~[u8]
}

impl Request {
    /// Create a new Request
    /// # Arguments
    /// * `url` -   the URL of the request, properly escaped
    /// * `headers` - the HTTP headers you choose to use
    /// * `body` - the body of the request
    /// # Example
    /// ~~~ {.rust}
    /// use std::hashmap::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert(headers::request::ACCEPT.to_owned(),~"application/json");
    ///
    /// let req = Request::new(url.to_owned(),headers,~[]);
    /// ~~~
    pub fn new(url: ~str, headers: Headers, body: ~[u8]) -> Request {
        Request {url: url, headers: headers, body: body}
    }
}

