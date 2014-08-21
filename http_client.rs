use std::collections::hashmap::HashMap;
use libc::{size_t,c_char};
use std::mem;

use curl::*;
use curl::callback::*;
use request::*;
use response::Response;
use curl::curl_ll::{curl_slist,curl_slist_append,curl_slist_free_all};

struct HttpHeaders {
    map: HashMap<String,String>
}

impl HttpHeaders {
    fn new() -> HttpHeaders {
        HttpHeaders { map: HashMap::new() }
    }
}

impl CurlCallback<c_char, HashMap<String,String>> for HttpHeaders {
    fn curl_get_userdata<'a>(&'a self) -> &'a HashMap<String,String> {
        &'a self.map
    }

    fn curl_get_callback(&self) -> CurlCallbackType<c_char, HashMap<String,String>> {
        unsafe {
            mem::transmute(c_curl_http_header_fn)
        }
    }
}

/// Rather opaque struct serving as HttpClient
#[deriving(Clone)]
pub struct HttpClient {
    curl: Curl
}

impl HttpClient {
    /// Return a new HttpClient object
    /// # Example
    /// ~~~ {.rust}
    /// let client = HttpClient::new();
    /// ~~~
    pub fn new() -> HttpClient {
        let cl = HttpClient { curl: Curl::new() };
        cl.curl.easy_setopt(FollowLocation(true));
        cl
    }

    /// Execute the given request
    /// # Arguments
    /// * `req` -   request to be executed
    /// # Example
    /// ~~~ {.rust}
    /// use headers;
    ///
    /// let client = HttpClient::new();
    ///
    /// let url = "http://api.4chan.org/pol/threads.json";
    /// let mut headers = HashMap::new();
    /// headers.insert(headers::request::ACCEPT.to_string(),~"application/json");
    ///
    /// let req = Request::new(url.to_string(),headers,~[]);
    ///
    /// let resp_res = client.exec(&req);
    ///
    /// match resp_res {
    ///     Ok(_) => { ; }
    ///     Err(msg) => { fail!("Error" + msg); }
    /// };
    /// ~~~
    pub fn exec(&self, req: &Request) -> Result<Response,String> {
        let url = req.url.to_str();
        let body = SimpleCurlByteBuffer::new();
        let headers = HttpHeaders::new();

        self.curl.easy_setopt(URL(url.as_slice()));
        self.curl.easy_setopt_callback(opt::WRITEDATA, opt::WRITEFUNCTION, &body);
        self.curl.easy_setopt_callback(opt::HEADERDATA, opt::HEADERFUNCTION, &headers);

        // FIXME setting headers like this is somewhat nasty - fix this with chaining or something
        let mut list = 0 as *curl_slist;
        if !req.headers.is_empty() {
            unsafe {
                for (k, v) in req.headers.iter() {
                    let h = format!("{}: {}",*k,*v);

                    h.with_c_str(|s| {
                        list = curl_slist_append(list,s);
                    });
                }
                self.curl.easy_setopt(UnsafeStringList(opt::HTTPHEADER, list));
            }
        }

        // Do the request
        let err = self.curl.easy_perform();
        
        if list as uint != 0 {
            unsafe {
                curl_slist_free_all(list);
            }
        }

        if err != code::CURLE_OK {
            return Err(easy_strerror(err));
        }

        let resp = Response::new(headers.map,body.data);

        // make sure to reset options for next request
        self.curl.easy_reset();

        Ok(resp)
    }
}

/// Callback called by libcurl when it receives another header
/// # Arguments
/// * `data` - the data received from this call
/// * `size` - the size each chunk received
/// * `nmemb` - the number of chunks
/// * `user_data` - pointer to user_data set when you set up a CurlCallback
/// # Safety Notes
/// the size of the header data received is (size * nmemb), and in this case
/// you should set user_data to be a reference to a `HashMap<~str,~str>`
/// although you can write such a function yourself that has different user data
extern "C" fn c_curl_http_header_fn (data: *c_char, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
    use std::str::raw::from_buf_len;
    use std::str::*;

    let head = unsafe { from_buf_len(data as *u8,(size * nmemb) as uint) };

    let colon = match head.as_slice().find(':') {
        Some(t) => t,
        None => { return size * nmemb; },
    };

    let (name, value) = (head.as_slice().slice(0,colon), head.as_slice().slice(colon + 2 ,head.len() - 1) );
    if name == "Set-Cookie" { return size * nmemb; }

    let h: &mut HashMap<String,String> = unsafe { mem::transmute(user_data) };
    h.insert(name.to_string(),value.to_string());
    size * nmemb
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hashmap::HashMap;
    use request::Request;

    #[test]
    fn test_basic_client() {
        use headers;

        let client = HttpClient::new();

        let url = "http://api.4chan.org/pol/threads.json";
        let mut headers = HashMap::new();
        headers.insert(headers::request::ACCEPT.to_string(),"application/json".to_str());

        let req = Request::new(url.to_string(),HashMap::new(),vec![]);

        let resp_res = client.exec(&req);

        match resp_res {
            Ok(response) => { 
                assert!(!response.headers.is_empty());
                assert!(!response.body.is_empty());
            }
            Err(msg) => { fail!("Error" + msg); }
        };
    }
}
