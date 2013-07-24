use curl;
use curl::callback::{CurlCallback, CurlCallbackType};
use std::libc::{size_t};
use std::cast;

/// This function is an example of the simplest functionality
pub fn example_http_get() {
    use std::str::from_bytes;

    let data_res = curl::get("http://api.4chan.org/pol/threads.json");

    match data_res {
        Ok(data) => { println(from_bytes(data)); }
        Err(msg) => { fail!("Error" + msg); }
    };
}

/// This function is an example of the http_client usage
pub fn example_http_basic_client() {
    use http_client::HttpClient;
    use std::hashmap::HashMap;
    use std::str::from_bytes;
    use request::Request;

    let client = HttpClient::new();

    let url = "http://api.4chan.org/pol/threads.json";

    let req = Request::new(url.to_owned(),HashMap::new(),~[]);

    let resp_res = client.exec(&req);

    match resp_res {
        Ok(data) => { println(from_bytes(data.body)); }
        Err(msg) => { fail!("Error" + msg); }
    };
}

/// A bit more advanced http_client usage
pub fn example_client_more() {
    use http_client::HttpClient;
    use std::hashmap::HashMap;
    use std::str::from_bytes;
    use request::Request;
    use headers;

     let client = HttpClient::new();

    let url = "http://api.4chan.org/pol/threads.json";
    let mut headers = HashMap::new();
    headers.insert(headers::request::ACCEPT.to_owned(),~"application/json");

    let req = Request::new(url.to_owned(),headers,~[]);

    let resp_res = client.exec(&req);

    match resp_res {
        Err(msg) => { fail!("Error" + msg); }
        Ok(resp) => {
            for resp.headers.iter().advance | (k, v) | {
                println(fmt!("%s: %s",*k,*v));
            }

            println(from_bytes(resp.body));
        }
    };
}

// The following examples are how to directly use the curl wrapper
// It is a very thin wrapper basically the curl_easy* interface
// You should check the curl_easy* docs located here:
// http://curl.haxx.se/libcurl/c/libcurl-easy.html

/// This function is passed as the WRITEFUNCTION variable in curl::easy_setopt.
/// It's a simple demo. You can reimplement similar functions as needed
/// in curl::easy_setopt
pub extern "C" fn write_fn (data: *u8, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
    use std::vec::raw::from_buf_raw;

    let s: &mut ~[u8] = unsafe { cast::transmute(user_data) };
    let new_data = unsafe { from_buf_raw(data, (size * nmemb) as uint) };
    s.push_all_move(new_data);
    size * nmemb
}

/// Example buffer struct that will be used for the callback function
struct ExampleWriteBuf {
    data: ~[u8]
}

/// Example of how to implement the CURL callback interface using a the CurlcCallback trait for a write function
impl CurlCallback<u8, ~[u8]> for ExampleWriteBuf {
    fn curl_get_userdata<'a>(&'a self) -> &'a ~[u8] {
        &'a self.data
    }

    fn curl_get_callback(&self) -> CurlCallbackType<u8, ~[u8]> {
        unsafe {
            cast::transmute(write_fn)
        }
    }
}

/// This shows some very basic usage of the curl_easy* interface
pub fn example_http_easy_basic_functionality() {
    use curl::Curl;
    use curl::code;
    use curl::opt;
    use std::str::from_bytes;

    let curl = Curl::new();
    let buf = ExampleWriteBuf { data: ~[] };

    curl.easy_setopt(curl::URL("www.google.com"));
    curl.easy_setopt_callback(opt::WRITEDATA, opt::WRITEFUNCTION, &buf);

    let err = curl.easy_perform();

    match err {
        code::CURLE_OK => {
            println(from_bytes(buf.data));
        }
        _ => { fail!(curl::easy_strerror(err)); }
    }
}
