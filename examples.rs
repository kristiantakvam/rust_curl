use curl;
use request;
use response;
use http_client;
use std::hashmap::HashMap;
use std::libc::{size_t,c_char};
use std::cast::transmute;

// this function is an example of the simplest functionality
pub fn example_get() {
	use std::str::from_bytes;
	
	let data_res = curl::get("http://api.4chan.org/pol/threads.json");
	
	match data_res {
		Ok(data) => { println(from_bytes(data)); }
		Err(msg) => { fail!("Error" + msg); }
	};
}

// this function is an example of the http_client usage
pub fn example_basic_client() {
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

// a bit more advanced http_client usage
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
    
    let req = Request::new(url.to_owned(),HashMap::new(),~[]);
    
    let resp_res = client.exec(&req);
    
    match resp_res {
		Err(msg) => { fail!("Error" + msg); }
		Ok(resp) => {
			for resp.headers.iter().advance | (&k, &v) | {
				println(fmt!("%s: %s",k,v));
			}
			
			println(from_bytes(resp.body)); 
		}
	};
}

// The following examples are how to directly use the curl wrapper
// It is a very thin wrapper basically the curl_easy* interface
// You should check the curl_easy* docs located here:
// http://curl.haxx.se/libcurl/c/libcurl-easy.html

// This function is passed as the WRITEFUNCTION variable
// in curl::easy_setopt
extern "C" fn write_fn (data: *u8, size: size_t, nmemb: size_t, user_data: *()) -> size_t {
    use std::vec::raw::from_buf_raw;
    
    let s: &mut ~[u8] = unsafe { transmute(user_data) };
    let new_data = unsafe { from_buf_raw(data, (size * nmemb) as uint) };
    s.push_all_move(new_data);
    size * nmemb
}

// This function is passed as the HEADERFUNCTION variable
// in curl::easy_setopt
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

// this shows some very basic usage of the curl_easy* interface
pub fn example_basic_functionality() {
	use curl::Curl;
	use curl::code;
	use curl::opt;

    let curl = Curl::new();
    do "www.google.com".as_c_str |c_str| { curl.easy_setopt(opt::URL,c_str); }
    curl.easy_setopt(opt::HEADER,1);
    curl.easy_setopt(opt::WRITEFUNCTION,write_fn);
    let s = ~"";
    curl.easy_setopt(opt::WRITEDATA, &s);
    let err = curl.easy_perform();
    
    match err {
    	code::CURLE_OK => {
    		println(s);
		}
		_ => { fail!(curl::easy_strerror(err)); }
	}
}

// This shows how you'd get headers with curl
fn example_get_headers() {
	use curl::Curl;
	use curl::code;
	use curl::opt;

    let curl = Curl::new();
    do "www.google.com".as_c_str |c_str| { curl.easy_setopt(opt::URL,c_str); }    
    
    curl.easy_setopt(opt::WRITEFUNCTION,write_fn);
    let s = ~"";
    curl.easy_setopt(opt::WRITEDATA, &s);
    
    curl.easy_setopt(opt::HEADERFUNCTION,header_fn);
    let headers: HashMap<~str,~str> = HashMap::new();
    curl.easy_setopt(opt::HEADERDATA,&headers);
    
    let err = curl.easy_perform();
    
    match err {
    	code::CURLE_OK => {
    		for headers.iter().advance | (&k, &v) | {
				println(fmt!("%s: %s",k,v));
			}
    		println(s);
		}
		_ => { fail!(curl::easy_strerror(err)); }
	}
}
