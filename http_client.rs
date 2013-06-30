use std::hashmap::HashMap;
use std::libc::{size_t,c_char};
use std::cast::transmute;

use curl::*;
use request::*;
use response::Response;

/// Rather opaque struct serving as HttpClient
#[deriving(Clone)]
pub struct HttpClient {
    priv curl: Curl
}

impl HttpClient {
    /// Return a new HttpClient object
    /// # Example
    /// ~~~ {.rust}
    /// let client = HttpClient::new();
    /// ~~~
    pub fn new() -> HttpClient {
        let cl = HttpClient { curl: Curl::new() };
        cl.curl.easy_setopt(opt::FOLLOWLOCATION,1);
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
    /// headers.insert(headers::request::ACCEPT.to_owned(),~"application/json");
    ///
    /// let req = Request::new(url.to_owned(),headers,~[]);
    ///
    /// let resp_res = client.exec(&req);
    ///
    /// match resp_res {
    ///     Ok(_) => { ; }
    ///     Err(msg) => { fail!("Error" + msg); }
    /// };
    /// ~~~
    pub fn exec(&self, req: &Request) -> Result<Response,~str> {
        let url = req.url.to_str();
        do url.as_c_str |c_str| { self.curl.easy_setopt(opt::URL,c_str); }

        self.curl.easy_setopt(opt::WRITEFUNCTION,write_fn);
        let body = ~[];
        self.curl.easy_setopt(opt::WRITEDATA, &body);

        self.curl.easy_setopt(opt::HEADERFUNCTION,header_fn);
        let headers: HashMap<~str,~str> = HashMap::new();
        self.curl.easy_setopt(opt::HEADERDATA,&headers);

        let err = match req.headers.is_empty() {
            true => { self.curl.easy_perform() }
            false => {
                unsafe {
                    let mut list = 0 as *curl_slist;

                    for req.headers.iter().advance |(&k, &v)| {
                        let h = fmt!("%s: %s",k,v);

                        do h.as_c_str |s| {
                            list = curl_slist_append(list,s);
                        }
                    }

                    self.curl.easy_setopt(opt::HTTPHEADER,list);
                    let rc = self.curl.easy_perform();
                    curl_slist_free_all(list);
                    rc
                }
            }
        };

        if err != code::CURLE_OK {
            return Err(easy_strerror(err));
        }

        let resp = Response::new(headers,body);

        Ok(resp)
    }
}

extern "C" fn write_fn (data: *u8, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
    use std::vec::raw::from_buf_raw;

    let s: &mut ~[u8] = unsafe { transmute(user_data) };
    let new_data = unsafe { from_buf_raw(data, (size * nmemb) as uint) };
    s.push_all_move(new_data);
    size * nmemb
}



extern "C" fn header_fn (data: *c_char, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
    use std::str::raw::from_c_str_len;
    use std::str::*;

    let head = unsafe { from_c_str_len(data,(size * nmemb) as uint) };

    let colon_res = head.find(':');
    if colon_res.is_none() { return size * nmemb; }

    let colon = colon_res.get();
    let (name, value) = (head.slice(0,colon), head.slice(colon + 2 ,head.len() - 1) );
    if name == "Set-Cookie" { return size * nmemb; }

    let h: &mut HashMap<~str,~str> = unsafe { transmute(user_data) };
    h.insert(name.to_owned(),value.to_owned());
    size * nmemb
}

#[test]
fn test_basic_client() {
    use headers;

    let client = HttpClient::new();

    let url = "http://api.4chan.org/pol/threads.json";
    let mut headers = HashMap::new();
    headers.insert(headers::request::ACCEPT.to_owned(),~"application/json");

    let req = Request::new(url.to_owned(),HashMap::new(),~[]);

    let resp_res = client.exec(&req);

    match resp_res {
        Ok(_) => { ; }
        Err(msg) => { fail!("Error" + msg); }
    };
}
