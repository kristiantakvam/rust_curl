use headers::Headers;

/// Represents an HTTP response
#[deriving(ToStr)]
pub struct Response {
    headers: Headers,
    body: ~[u8]
}

impl Response {
	/// Creates a new response struct
	/// # Arguments
	/// * `headers` -	the HTTP headers from the response
	/// * `body` -	the body of the HTTP response
	/// # Example
	/// ~~~ {.rust}
	/// use std::str::from_bytes
	///
	/// let client = HttpClient::new();
    ///
    /// let url = "http://api.4chan.org/pol/threads.json";
    ///
    /// let req = Request::new(url.to_owned(),HashMap::new(),~[]);
    ///
    /// let resp_res: Result<Response,~str> = client.exec(&req);
    /// 
    /// match resp_res {
	/// 	code::CURLE_OK => {
    /// 		for resp.headers.iter().advance | (&k, &v) | {
	/// 			println(fmt!("%s: %s",k,v));
	/// 		}
    /// 		println(from_bytes(resp.bod));
	/// 	}
	/// 	Err(msg) => { fail!("Error" + msg); }
	/// };
	/// ~~~
    pub fn new(headers: Headers, body: ~[u8]) -> Response {
        Response {headers: headers, body: body}
    }
}
