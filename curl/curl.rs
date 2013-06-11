use std::libc::{size_t,c_int,c_void,c_char,c_longlong};
use std::cast::transmute;
use std::hashmap::HashMap;

pub mod opt;
pub mod code;

type CURL = ();
type CURLcode = c_int;
type CURLINFO = c_int;

struct curl_slist {
    data: *c_char,
    next: *curl_slist
}

#[link_args = "-lcurl"]
extern {
    fn curl_easy_cleanup(handle: *CURL) -> c_void;
    fn curl_easy_duphandle(handle: *CURL) -> *CURL;
    fn curl_easy_escape(curl: *CURL, url: *c_char, length: c_int) -> *c_char;
    // Skipping get_info
    fn curl_easy_init() -> *CURL;
    fn curl_easy_perform(curl: *CURL) -> CURLcode;
    // Skipping curl_easy_recv
    fn curl_easy_reset(curl: *CURL) -> c_void;
    // Skipping curl_easy_send
    fn curl_easy_setopt(handle: *CURL, opt: c_int, val: c_longlong) -> CURLcode;
    fn curl_easy_strerror(err: CURLcode) -> *c_char;
    fn curl_easy_unescape(curl: *CURL, url: *c_char, inlength: c_int, outlength: *c_int) -> *c_char;
    fn curl_free(ptr: *c_char) -> c_void;
    
    fn curl_slist_append(list: *curl_slist, s: *c_char) -> *curl_slist;
    fn curl_slist_free_all(list: *curl_slist) -> c_void;
}

#[deriving(Eq)]
pub struct Curl {
    priv curl: *CURL
}

impl Curl {
    pub fn new() -> Curl {
        unsafe {
            Curl {curl: curl_easy_init() }
        }
    }
    
    pub fn easy_escape(&self, url: &str) -> ~str {
        use std::str::raw::from_c_str;
        
        let len = url.len() as c_int;
        do url.as_c_str |s| {
            unsafe {
                let raw = curl_easy_escape(self.curl,s,len);
                let ret = from_c_str(raw);
                curl_free(raw);
                ret
            }
        }
    }
    
    pub fn easy_unescape(&self, s: &str) -> ~str {
        use std::str::raw::from_c_str_len;
        
        do s.as_c_str |c_str| {
            unsafe {
                let in_len = s.len() as c_int;
                let out_len = &(0 as c_int);
                let raw = curl_easy_unescape(self.curl, c_str, in_len, out_len);
                let ret = from_c_str_len(raw,*out_len as uint);
                curl_free(raw);
                ret
            }
        }
    }           
    
    pub fn easy_setopt<T>(&self, opt: i32, val: T) -> code::Code {
        unsafe {
            let opt_val = transmute(val);
            let raw_code = curl_easy_setopt(self.curl, opt, opt_val);
            transmute(raw_code as i64)
        }
    }
    
    pub fn easy_perform(&self) -> code::Code {
        unsafe {
            let raw_code = curl_easy_perform(self.curl);
            transmute(raw_code as i64)
        }
    }
    
    pub fn easy_reset(&self) {
        unsafe {
            curl_easy_reset(self.curl);
        }
    }
    
    pub fn easy_strerror(c: code::Code) -> ~str {
        use std::str::raw::from_c_str;
        
        unsafe {
            let c32: i32 = transmute::<code::Code,i64>(c).to_i32();
            let raw = curl_easy_strerror(c32);
            let ret = from_c_str(raw);
            ret
        }
    }
    
    pub fn add_headers(&self, headers: &HashMap<~str,~str>) -> code::Code {
        unsafe {
            let mut list = 0 as *curl_slist;
        
            for headers.each |&k, &v| {
                let h = fmt!("%s: %s",k,v);
                do h.as_c_str |s| {
                    list = curl_slist_append(list,s);
                }
            }
        
            if list != 0 as *curl_slist { 
                curl_slist_free_all(list); 
            }
            self.easy_setopt(opt::HTTPHEADER,list)
        }
    }                                         
            
}

impl Clone for Curl {
    pub fn clone(&self) -> Curl {
        unsafe {
            Curl {curl: curl_easy_duphandle(self.curl) }
        }
    }
}

impl Drop for Curl {
    #[unsafe_destructor]
    pub fn finalize(&self) {
        unsafe {
            curl_easy_cleanup(self.curl);
        }
    }
}

extern "C" fn write_fn (data: *c_char, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
    use std::str::raw::from_c_str_len;
    use std::str::*;
    
    let s: &mut ~str = unsafe { transmute(user_data) };
    unsafe { push_str(s, from_c_str_len(data,(size * nmemb) as uint)); }
    size * nmemb
}

extern "C" fn header_fn (data: *c_char, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
    use std::str::raw::from_c_str_len;
    use std::str::*;
    
    let head = unsafe { from_c_str_len(data,(size * nmemb) as uint) };
    
    let colon_res = do find(head) |c| { c == ':' };
    if colon_res.is_none() { return size * nmemb; }
    
    let colon = colon_res.get();
    let (name, value) = (head.substr(0,colon), head.substr(colon + 2 ,head.len() - colon - 3) );
    if name == "Set-Cookie" { return size * nmemb; }
    
    let h: &mut HashMap<~str,~str> = unsafe { transmute(user_data) };
    h.insert(name.to_owned(),value.to_owned());
    size * nmemb
}

#[test]
fn test_init_clone() {
    let c1 = Curl::new();
    let c2 = c1.clone();
    assert!(c1 != c2);
}

#[test]
fn test_easy_escape() {
    let c1 = Curl::new();

    let query = ~"lol and stuff";
    let escaped_query = c1.easy_escape(query);
    assert!(escaped_query == ~"lol%20and%20stuff");
    let unescaped_query = c1.easy_unescape(escaped_query);
    assert!(unescaped_query == query);
}

#[test]
fn test_basic_functionality() {
    let curl = Curl::new();
    do "www.google.com".as_c_str |c_str| { curl.easy_setopt(opt::URL,c_str); }
    curl.easy_setopt(opt::HEADER,1);
    curl.easy_setopt(opt::WRITEFUNCTION,write_fn);
    let s = ~"";
    curl.easy_setopt(opt::WRITEDATA, &s);
    let err = curl.easy_perform();
    assert!(err == code::CURLE_OK);
}

#[test]
fn test_get_headers() {
    let curl = Curl::new();
    do "www.google.com".as_c_str |c_str| { curl.easy_setopt(opt::URL,c_str); }    
    
    curl.easy_setopt(opt::WRITEFUNCTION,write_fn);
    let s = ~"";
    curl.easy_setopt(opt::WRITEDATA, &s);
    
    curl.easy_setopt(opt::HEADERFUNCTION,header_fn);
    let headers: HashMap<~str,~str> = HashMap::new();
    curl.easy_setopt(opt::HEADERDATA,&headers);
    
    let err = curl.easy_perform();
    assert!(err == code::CURLE_OK);
}

#[test]
fn test_add_headers() {
    use super::headers;
    
    println("IN");
    let curl = Curl::new();
    let mut headers = HashMap::new();
    headers.insert(headers::request::ACCEPT.to_owned(), ~"text/plain");
    
    let err = curl.add_headers(&headers);
    assert!(err == code::CURLE_OK);
}
