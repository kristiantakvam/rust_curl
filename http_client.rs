use std::hashmap::HashMap;
use std::libc::{size_t,c_char};
use std::cast::transmute;

use curl::*;
use request::*;
use response::Response;

#[deriving(Clone)]
pub struct HttpClient {
    priv curl: Curl
}

impl HttpClient {
    pub fn new() -> HttpClient {
        let cl = HttpClient { curl: Curl::new() };
        cl.curl.easy_setopt(opt::FOLLOWLOCATION,1);
        cl
    }

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
                    
                    for req.headers.each |&k, &v| {
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
            return Err(Curl::easy_strerror(err));
        }
        
        let resp = Response::new(headers,body);
        
        Ok(resp)
    }
}

extern "C" fn write_fn (data: *u8, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
    use std::vec::raw::from_buf_raw;
    
    let body: &mut ~[u8] = unsafe { transmute(user_data) };
    unsafe { body.push_all_move(from_buf_raw(data,(size * nmemb) as uint)); }
    size * nmemb
}

extern "C" fn header_fn (data: *c_char, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
    use std::str::raw::from_c_str_len;
    use std::str::*;
    
    let head = unsafe { from_c_str_len(data,(size * nmemb) as uint) };
    
    let colon_res = head.find(':');
    if colon_res.is_none() { return size * nmemb; }
    
    let colon = colon_res.get();
    let (name, value) = (head.substr(0,colon), head.substr(colon + 2 ,head.len() - colon - 3) );
    if name == "Set-Cookie" { return size * nmemb; }
    
    let h: &mut HashMap<~str,~str> = unsafe { transmute(user_data) };
    h.insert(name.to_owned(),value.to_owned());
    size * nmemb
}

#[test]
fn test_basic_client() {
    use std::str::from_bytes;
    use headers;
    
    let client = HttpClient::new();
    
    let url = "http://api.4chan.org/pol/threads.json";
    let mut headers = HashMap::new();
    headers.insert(headers::request::ACCEPT.to_owned(),~"application/json");
    
    let req = Request::new(url.to_owned(),HashMap::new(),~[]);
    
    let resp = client.exec(&req).get();
    
    for resp.headers.each |&k, &v| {
        println(fmt!("%s: %s",k,v));
    }
    
    println(from_bytes(resp.body));
}
