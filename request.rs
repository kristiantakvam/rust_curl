use headers::Headers;

/// Represents HTTP request methods
pub enum Method {
    GET,
    POST
}

/// The HTTP request struct
#[deriving(Show)]
pub struct Request {
    pub url: String,
    pub headers: Headers,
    body: Vec<u8>
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
    /// headers.insert(headers::request::ACCEPT.to_string(),~"application/json");
    ///
    /// let req = Request::new(url.to_string(),headers,~[]);
    /// ~~~
    pub fn new(url: String, headers: Headers, body: Vec<u8>) -> Request {
        Request {url: url, headers: headers, body: body}
    }
}

