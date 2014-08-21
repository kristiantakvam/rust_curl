rust_curl
=========

Basic rust wrapper over the curl_easy* interface. Also a very very tiny http client

To build it type ```make``` (or ```rustc rust_curl.rc -O``` if you do not have GNU make installed).

If you want to build it with a main function instead of as a library, use
    ```rustc rust_curl.rc -O --bin``` to see the example output provided
    by the program below.

Here is example usage of the laughable "HTTP client" included:

```
    use std::hashmap::HashMap;
    use request::Request;
    use std::str::from_bytes;
    
    let client = http_client::HttpClient::new();
    
    let url = "http://api.4chan.org/pol/threads.json";
    let mut headers = HashMap::new();
    headers.insert(headers::request::ACCEPT.to_string(),~"application/json");

    let req = Request::new(url.to_string(),headers,~[]);
    
    let resp = client.exec(&req).get();
    
    for resp.headers.each |&k, &v| {
        println(fmt!("%s: %s",k,v));
    }
    
    println(from_bytes(resp.body));
```

*FIXME* this is now out of date.

The client is very basic, ignores any cookies that are received (although
    technically you can still send cookies, by adding the "Cookie" header to
    the request HashMap) and so is only suitable for the most basic of use
    cases. I do plan to improve it in the future though.
    
If you have more complicated use patterns, you will probably want to directly
    use the underlying curl wrapper. You will essentially be using the curl API
    and so I suggest visiting their docs at: 
        http://curl.haxx.se/libcurl/c/libcurl-easy.html

The wrapper I provide around curl is pretty thin, so you should be able to
    do just about anything you could using the C curl easy_* interface. Below is
    the code for the ```HttpClient::get(&self, req: &Request)``` method
    
```
pub fn exec(&self, req: &Request) -> Result<Response,~str> {
        let url = req.url.to_str();
        self.curl.easy_setopt_str(opt::URL, url);
        
        let body = SimpleCurlByteBuffer::new();
        let headers = HttpHeaders::new();
        
        self.curl.easy_setopt_callback(opt::WRITEDATA, opt::WRITEFUNCTION, &body);
        self.curl.easy_setopt_callback(opt::HEADERDATA, opt::HEADERFUNCTION, &headers);
        
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
                    
                    self.curl.easy_setopt_slist(opt::HTTPHEADER, list):q;
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
```

Two main things to take notice of. The ```Curl::easy_setopt(&self, val: T)```
    is generic, as it forwards it's arguments to ```curl_easy_setopt```
    which is a C function that can take either a pointer to a function,
    user supplied data for a Curl callback, a 32bit int, or a 64bit int.

It is up to you to take care of the lifetimes and types of any objects
    you give this function. You will see the lifetime management I have to
    do with the ```curl_slist``` of headers that I pass in, for example.
    Make sure any functions that you give as an argument have been declared
    as ```extern C```. Here are the two such functions I used above:
    
```
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
    h.insert(name.to_string(),value.to_string());
    size * nmemb
}
```
