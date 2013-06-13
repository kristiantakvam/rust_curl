rust_curl
=========

Basic rust wrapper over the curl_easy* interface. Also a very very tiny http client

Here is example usage of the laughable "HTTP client" included:

```
    use std::hashmap::HashMap;
    use request::Request;
    use std::str::from_bytes;
    
    let client = http_client::HttpClient::new();
    
    let url = "http://api.4chan.org/pol/threads.json";
    let mut headers = HashMap::new();
    headers.insert(headers::request::ACCEPT.to_owned(),~"application/json");

    let req = Request::new(url.to_owned(),headers,~[]);
    
    let resp = client.exec(&req).get();
    
    for resp.headers.each |&k, &v| {
        println(fmt!("%s: %s",k,v));
    }
    
    println(from_bytes(resp.body));
```

The client is very basic, ignores any cookies that are received (although)
  technically you can still send cookies, by adding the "Cookie" header to
  the request HashMap.
