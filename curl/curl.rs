use std::libc::{size_t,c_int,c_void,c_char};
use std::cast::transmute;
use std::hashmap::HashMap;

pub mod opt;
pub mod code;

type CURL = ();
type CURLcode = c_int;
type CURLINFO = c_int;

/// This is a the curl_slist structure, a list of c strings
/// 
/// This is a simple singly-linked list of c_strings
/// 	that is obviously an unsafe structure which should
///		be used properly and cautiously
pub struct curl_slist {
    data: *c_char,
    next: *curl_slist
}

#[link_args = "-lcurl"]
extern {
    pub fn curl_easy_cleanup(handle: *CURL) -> c_void;
    pub fn curl_easy_duphandle(handle: *CURL) -> *CURL;
    pub fn curl_easy_escape(curl: *CURL, url: *c_char, length: c_int) -> *c_char;
    // Skipping get_info
    pub fn curl_easy_init() -> *CURL;
    pub fn curl_easy_perform(curl: *CURL) -> CURLcode;
    // Skipping curl_easy_recv
    pub fn curl_easy_reset(curl: *CURL) -> c_void;
    // Skipping curl_easy_send
    pub fn curl_easy_setopt(handle: *CURL, opt: c_int, val: *c_void) -> CURLcode;
    pub fn curl_easy_strerror(err: CURLcode) -> *c_char;
    pub fn curl_easy_unescape(curl: *CURL, url: *c_char, inlength: c_int, outlength: *c_int) -> *c_char;
    pub fn curl_free(ptr: *c_char) -> c_void;
    
    pub fn curl_slist_append(list: *curl_slist, s: *c_char) -> *curl_slist;
    pub fn curl_slist_free_all(list: *curl_slist) -> c_void;
}

/// This is a an opaque wrapper over the equally opaque
/// CURL pointer.
#[deriving(Eq)]
pub struct Curl {
    priv curl: *CURL
}

impl Curl {
	/// Return a new Curl object
	/// # Example
	/// ~~~ {.rust}
	/// let curl = Curl::new();
	/// ~~~
    pub fn new() -> Curl {
        unsafe {
            Curl {curl: curl_easy_init()}
        }
    }
    
    /// URL-escape a string 
    /// # Arguments
    /// * `url` -	String to be escaped
    /// # Safety Note 
    /// Not to be used on an entire string
    /// # Example
    /// ~~~ {.rust}
    /// let curl = Curl::new();
    /// let query = ~"lol and stuff";
    /// let escaped = curl.easy_escape(query);
    /// ~~~
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
    
    /// un-URL-escape a string
    /// # Arguments
    /// * `s` -	String to be unescaped
    /// # Safety Note 
    /// Not to be used on an entire string
    /// # Example
    /// ~~~ {.rust}
    /// let curl = Curl::new();
    /// let escaped = ~"lol%20and%20stuff";
    /// let unescaped = curl.easy_unescape(escaped);
    /// ~~~
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
    
    /// Wrapper over the easy_setopt function, which will be called
    /// before calling calling easy_perform.
    /// # Arguments
    /// * `opt` - option to be set
    /// * `val` - value of the option being set
    /// # Safety Note
    /// The opt arguments should be one of the values from curl::opt::*;
    /// The val argument can be either a pointer to a function, user 
    /// supplied data for a Curl callback, a 32bit int, or a 64bit int.
    /// # Example
    /// ~~~ {.rust}
    /// let curl = Curl::new();
    /// curl.easy_setopt(opt::HEADER,1);
    /// ~~~
    pub fn easy_setopt<T>(&self, opt: i32, val: T) -> code::Code {
        unsafe {
            let opt_val = transmute(val);
            let raw_code = curl_easy_setopt(self.curl, opt, opt_val);
            transmute(raw_code as i64)
        }
    }
    /// Wrapper over curl_easy_perform (performs the request).
    /// # Example
    /// ~~~ {.rust}
    /// let curl = Curl::new();
    /// do "www.google.com".as_c_str |c_str| { curl.easy_setopt(opt::URL,c_str); }
    /// curl.easy_setopt(opt::HEADER,1);
    /// curl.easy_setopt(opt::WRITEFUNCTION,my_write_fn);
    /// curl.easy_perform();
    /// ~~~
    pub fn easy_perform(&self) -> code::Code {
        unsafe {
            
            let raw_code = curl_easy_perform(self.curl);
            
            transmute(raw_code as i64)
        }
    }
    
    /// Wrapper over curl_easy_reset, which clears all previously
    /// set options.
    /// # Example
    /// ~~~ {.rust}
    /// let curl = Curl::new();
    /// curl.easy_setopt(opt::HEADER,1);
    /// curl.easy_reset();
    /// ~~~
    pub fn easy_reset(&self) {
        unsafe {
            curl_easy_reset(self.curl);
        }
    }         
}

/// Converts a curl::code into a it's error string.
/// # Arguments
/// * `c` - code to get error string from
/// # Example
/// ~~~ {.rust}
/// let curl = Curl::new();
/// do "www.google.com".as_c_str |c_str| { curl.easy_setopt(opt::URL,c_str); }
/// // omitted a few easy_setopt calls, but you need to either set a WRITEFUNCTION
/// // or a FILE* as the WRITEDATA to avoid a segfault
/// let err = curl.easy_perform();
/// let err_str = easy_strerror(err);
/// ~~~
pub fn easy_strerror(c: code::Code) -> ~str {
    use std::str::raw::from_c_str;
    
    unsafe {
        let c32: i32 = transmute::<code::Code,i64>(c).to_i32();
        let raw = curl_easy_strerror(c32);
        let ret = from_c_str(raw);
        ret
    }
}

/// Convenience function to fetch the body of HTTP response at the
/// given URL. You are responsible for ensuring it's properly escaped/
/// # Arguments
/// * `url` - url to fetch body from
/// # Example
/// ~~~ {.rust}
/// use std::str::from_bytes;
///
/// let data_res = get("http://api.4chan.org/pol/threads.json");
///	
/// match data_res {
/// 	Ok(data) => { println(from_bytes(data)); }
/// 	Err(msg) => { fail!("Error" + msg); }
/// };
/// ~~~
pub fn get(url: &str) -> Result<~[u8],~str> {
	let curl = Curl::new();
	do url.as_c_str |c_str| { curl.easy_setopt(opt::URL,c_str); }
	curl.easy_setopt(opt::HEADER,1);
	curl.easy_setopt(opt::WRITEFUNCTION, write_fn);
	let data: ~[u8] = ~[];
	curl.easy_setopt(opt::WRITEDATA, &data);
	let err = curl.easy_perform();
	
	match err {
		code::CURLE_OK => { Ok(data) }
		_ => { Err(easy_strerror(err)) }
	}
}   

impl Clone for Curl {
    pub fn clone(&self) -> Curl {
        unsafe {
            Curl {curl: curl_easy_duphandle(self.curl)}
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

/// Write callback called by libcurl when it receives more data
/// # Arguments
/// * `data` - the data received from this call
/// * `size` - the size each chunk received
/// * `nmemb` - the number of chunks
/// * `user_data` - pointer to user_data set with a 
/// curl.easy_setopt(opt::WRITEDATA, my_data); call.
/// # Safety Notes
/// the size of the data received is (size * nmemb), and in this case
/// you should set user_data to be a reference to a ~[u8], although
/// you can write such a function yourself that has different user data
pub extern "C" fn write_fn (data: *u8, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
    use std::vec::raw::from_buf_raw;
    
    let s: &mut ~[u8] = unsafe { transmute(user_data) };
    let new_data = unsafe { from_buf_raw(data, (size * nmemb) as uint) };
    s.push_all_move(new_data);
    size * nmemb
}

/// Callback called by libcurl when it receives another header
/// # Arguments
/// * `data` - the data received from this call
/// * `size` - the size each chunk received
/// * `nmemb` - the number of chunks
/// * `user_data` - pointer to user_data set with a 
/// curl.easy_setopt(opt::HEADERDATA, my_data); call.
/// # Safety Notes
/// the size of the header data received is (size * nmemb), and in this case
/// you should set user_data to be a reference to a `HashMap<~str,~str>`
/// although you can write such a function yourself that has different user data
pub extern "C" fn header_fn (data: *c_char, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
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
fn test_init_clone() {
    let c1 = Curl::new();
    let c2 = c1.clone();

    assert!(c1.curl != c2.curl);
}

#[test]
fn test_easy_escape() {
    let c1 = Curl::new();

    let query = ~"lol and stuff";
    let escaped_query = c1.easy_escape(query);
    let unescaped_query = c1.easy_unescape(escaped_query);
    
    assert!(escaped_query == ~"lol%20and%20stuff");
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
    
    assert!(!s.is_empty());
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
    assert!(!headers.is_empty());
    assert!(!s.is_empty());
    assert!(err == code::CURLE_OK);
}

#[test]
fn test_simple_get() {
	let data_res = get("http://api.4chan.org/pol/threads.json");
	
	match data_res {
		Ok(_) => { ; }
		Err(msg) => { fail!("Error" + msg); }
	};
}
