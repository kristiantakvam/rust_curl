use std::libc::{c_int,c_void};
use std::cast;

use curl::curl_ll::*;
use curl::callback::{CurlCallback, SimpleCurlByteBuffer};

pub mod opt;
pub mod code;
pub mod curl_ll;
pub mod callback;

/// A set of options available to set on the curl 'request'. 
/// These generally map one-to-one to the Curl options available via curl_easy_setopt.
///
/// Currently, this option list is incomplete and only implements the things necessary for the HTTP examples 
/// and a tiny bit more.
///
/// # Example
/// ~~~ {.rust}
/// let opt = Username("alice");
/// ~~~
pub enum EasyCurlOption<'self> {
    Username(&'self str),
    Password(&'self str),
    Proxy(&'self str, Option<&'self str>, Option<&'self str>),
    URL(&'self str),

    Referer(&'self str),
    UnsafeStringList(opt::CURLoption, *curl_slist),

    Timeout(int),
    VerboseMode(bool),
    ShowHeaders(bool),
    FollowLocation(bool)
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
    /// * `url` -   String to be escaped
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
    /// * `s` - String to be unescaped
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

    /// Set an option (using the easy interface). Wraps the easy_setopt function.
    /// Options can affect a wide variety of outcomes (headers, proxy details, verbose mode)
    /// For full documentation refer to the CURL docs.
    ///
    /// # Arguments
    /// * `opt` - option (with value) to set
    ///
    /// # Example
    /// ~~~ {.rust}
    /// let curl = Curl::new();
    /// curl.easy_setopt(URL("http://google.com"));
    /// curl.easy_setopt(VerboseMode(true));
    /// ~~~
    pub fn easy_setopt<'a>(&self, opt: EasyCurlOption<'a>) -> code::CURLcode {
        match opt {
            FollowLocation(enable) => self.easy_setopt_bool(opt::FOLLOWLOCATION, enable),
            Password(pass) => self.easy_setopt_str(opt::PASSWORD, pass),
            Proxy(proxy, user, pass) => {
                self.easy_setopt_str(opt::PROXY, proxy);
                match user {
                    Some(u) => self.easy_setopt_str(opt::PROXYUSERNAME, u),
                    None => code::CURLE_OK
                };
                match pass {
                    Some(p) => self.easy_setopt_str(opt::PROXYPASSWORD, p),
                    None => code::CURLE_OK
                }
            },
            Referer(referer) => self.easy_setopt_str(opt::REFERER, referer),
            ShowHeaders(enable) => self.easy_setopt_bool(opt::HEADER, enable),
            Timeout(secs) => self.easy_setopt_long(opt::TIMEOUT, secs),
            UnsafeStringList(curlopt, slist) => self.easy_setopt_slist(curlopt, slist),
            URL(url) => self.easy_setopt_str(opt::URL, url),
            Username(user) => self.easy_setopt_str(opt::USERNAME, user),
            VerboseMode(enable) => self.easy_setopt_bool(opt::VERBOSE, enable),
        }
    }

    /// Set a callback option (wraps the easy_setopt function).
    ///
    /// Callbacks are pairs a functions returning a data value (buffer) and a pointer to a C-callable function
    /// that will be called by curl to act on the data.
    ///
    /// Callbacks are generally used to read/write data in a request (data that is not static in size).
    ///
    /// # Arguments
    /// * `dataOpt` - the CURL type for the data argument
    /// * `opt` - the CURL type for the callback function
    /// * `callback` - the encapsulated callback
    ///
    /// # Example
    /// ~~~ {.rust}
    /// let curl = Curl::new();
    /// let body = SimpleCurlByteBuffer::new();
    /// curl.easy_setopt_callback(opt::WRITEDATA, opt::WRITEFUNCTION, &body);
    /// ~~~    
    pub fn easy_setopt_callback<D, U, T: CurlCallback<D, U>>(&self, dataOpt: opt::CURLoption, 
        callbackOpt: opt::CURLoption, callback: &T) -> code::CURLcode {
        let data_val = callback.curl_get_userdata();
        let fn_val = callback.curl_get_callback();
        unsafe {           
            fail_on_curl_error(curl_easy_setopt(self.curl, dataOpt, cast::transmute(data_val)));
            fail_on_curl_error(curl_easy_setopt(self.curl, callbackOpt, cast::transmute(fn_val)));
        }
        code::CURLE_OK
    }
    
    /// Wrapper over curl_easy_perform (performs the request).
    /// # Example
    /// ~~~ {.rust}
    /// let curl = Curl::new();
    /// let my_callback = SimpleCurlByteBuffer::new();
    /// curl.easy_setopt_str(opt::URL, "www.google.com");
    /// curl.easy_setopt_long(opt::HEADER, 1);
    /// curl.easy_setopt_callback(opt::WRITEDATA, opt::WRITEFUNCTION, &my_callback);
    /// curl.easy_perform();
    /// ~~~
    pub fn easy_perform(&self) -> code::CURLcode {
        unsafe {
            curl_easy_perform(self.curl)
        }
    }

    /// Wrapper over curl_easy_reset, which clears all previously
    /// set options.
    /// # Example
    /// ~~~ {.rust}
    /// let curl = Curl::new();
    /// curl.easy_setopt_long(opt::HEADER,1);
    /// curl.easy_reset();
    /// ~~~
    pub fn easy_reset(&self) {
        unsafe {
            curl_easy_reset(self.curl);
        }
    }

    // TODO the below need to be checked against their option types to ensure no failure occurs

    fn easy_setopt_str(&self, opt: opt::CURLoption, string: &str) -> code::CURLcode {
        let c_str = string.as_c_str(|x|x);
        unsafe {
            fail_on_curl_error(curl_easy_setopt(self.curl, opt, c_str as *c_void))
        }
    }

    fn easy_setopt_slist(&self, opt: opt::CURLoption, val: *curl_slist) -> code::CURLcode {
        unsafe {
            let opt_val = cast::transmute(val);
            fail_on_curl_error(curl_easy_setopt(self.curl, opt, opt_val))
        }
    }

    #[inline]
    fn easy_setopt_bool(&self, opt: opt::CURLoption, val: bool) -> code::CURLcode {
        self.easy_setopt_long(opt, val as int)
    }

    fn easy_setopt_long(&self, opt: opt::CURLoption, val: int) -> code::CURLcode {
        unsafe {
            fail_on_curl_error(curl_easy_setopt(self.curl, opt, val as *c_void))
        }
    }
}


/// Function that fails on any non-OK status code from CURL
#[inline]
fn fail_on_curl_error(c: code::CURLcode) -> code::CURLcode {
    match c {
        code::CURLE_OK => c,
        _ => fail!(easy_strerror(c))
    }
}


/// Converts a curl::code into a it's error string.
/// # Arguments
/// * `c` - code to get error string from
/// # Example
/// ~~~ {.rust}
/// let curl = Curl::new();
/// curl.easy_setopt_str(opt::URL, "www.google.com");
/// // omitted a few easy_setopt calls, but you need to either set a WRITEFUNCTION
/// // or a FILE* as the WRITEDATA to avoid a segfault
/// let err = curl.easy_perform();
/// let err_str = easy_strerror(err);
/// ~~~
pub fn easy_strerror(c: code::CURLcode) -> ~str {
    use std::str::raw::from_c_str;

    unsafe {
        let raw = curl_easy_strerror(c);
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
///     Ok(data) => { println(from_bytes(data)); }
///     Err(msg) => { fail!("Error" + msg); }
/// };
/// ~~~
pub fn get(url: &str) -> Result<~[u8],~str> {
    let curl = Curl::new();

    let buf = SimpleCurlByteBuffer::new();

    curl.easy_setopt_str(opt::URL, url);
    curl.easy_setopt_callback(opt::WRITEDATA, opt::WRITEFUNCTION, &buf);

    let err = curl.easy_perform();

    match err {
        code::CURLE_OK => { Ok(buf.data) }
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
    pub fn drop(&self) {
        unsafe {
            curl_easy_cleanup(self.curl);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use curl::callback::{SimpleCurlByteBuffer};

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
        
        let buf = SimpleCurlByteBuffer::new();

        curl.easy_setopt_str(opt::URL, "www.google.com");
        curl.easy_setopt_long(opt::HEADER, 1);
        curl.easy_setopt_callback(opt::WRITEDATA, opt::WRITEFUNCTION, &buf);
        
        let err = curl.easy_perform();

        assert!(!buf.data.is_empty());
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

    #[test] #[should_fail]
    fn test_invalid_params_should_fail() {
        // create a NULL pointer manually and try to pass it into a setopt function
        use curl::curl_ll::CURL;
        let curl = Curl { curl: 0 as *CURL };

        // should fail in the curl library and cause task failure
        curl.easy_setopt_long(opt::HEADER, 1);
    }
}
