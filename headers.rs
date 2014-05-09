use collections::hashmap::HashMap;

/// This module contains static strings of the HTTP request headers
/// found on https://en.wikipedia.org/wiki/List_of_HTTP_header_fields#Requests
pub mod request {
    pub static ACCEPT: &'static str = "Accept";
    pub static ACCEPT_CHARSET: &'static str = "Accept-Charset";
    pub static ACCEPT_ENCODING: &'static str = "Accept-Encoding";
    pub static ACCEPT_LANGUAGE: &'static str = "Accept-Language";
    pub static ACCEPT_DATETIME: &'static str = "Accept-Datetime";
    pub static AUTHORIZATION: &'static str = "Authorization";
    pub static CACHE_CONTROL: &'static str = "Cache-Control";
    pub static CONNECTION: &'static str = "Connection";
    pub static CONTENT_LENGTH: &'static str = "Content-Length";
    pub static CONTENT_MD5: &'static str = "Content-MD5";
    pub static COOKIE: &'static str = "Cookie";
    pub static DATE: &'static str = "Date";
    pub static EXPECT: &'static str = "Expect";
    pub static FROM: &'static str = "From";
    pub static HOST: &'static str = "Host";
    pub static IF_MATCH: &'static str = "If-Match";
    pub static IF_MODIFIED_SINCE: &'static str = "If-Modified-Since";
    pub static MAX_FORWARDS: &'static str = "Max-Forwards";
    pub static ORIGIN: &'static str = "Origin";
    pub static PROXY_AUTHORIZATION: &'static str = "Proxy_Authorization";
    pub static RANGE: &'static str = "Range";
    pub static REFERRER: &'static str = "Referrer";
    pub static TE: &'static str = "TE";
    pub static UPGRADE: &'static str = "Upgrade";
    pub static USER_AGENT: &'static str = "User-Agent";
    pub static VIA: &'static str = "Via";
    pub static WARNING: &'static str = "Warning";
}

/// This module contains static strings of the HTTP response headers
/// found on https://en.wikipedia.org/wiki/List_of_HTTP_header_fields#Responses
pub mod response {
    pub static ACCESS_CONTROL_ALLOW_ORIGIN: &'static str = "Access-Control-Allow-Origin";
    pub static ACCEPT_RANGES: &'static str = "Accept-Ranges";
    pub static AGE: &'static str = "Age";
    pub static ALLOW: &'static str = "Allow";
    pub static CACHE_CONTROL: &'static str = "Cache-Control";
    pub static CONNECTION: &'static str = "Connection";
    pub static CONTENT_ENCODING: &'static str = "Content-Encoding";
    pub static CONTENT_LANGUAGE: &'static str = "Content-Language";
    pub static CONTENT_LENGTH: &'static str = "Content-Length";
    pub static CONTENT_LOCATION: &'static str = "Content-Location";
    pub static CONTENT_MD5: &'static str = "Content-MD5";
    pub static CONTENT_DISPOSITION: &'static str = "Content-Disposition";
    pub static CONTENT_RANGE: &'static str = "Content-Range";
    pub static CONTENT_TYPE: &'static str = "Conetent-Type";
    pub static DATE: &'static str = "Date";
    pub static ETAG: &'static str = "ETag";
    pub static EXPIRES: &'static str = "Expires";
    pub static LAST_MODIFIED: &'static str = "Last-Modified";
    pub static LINK: &'static str = "Link";
    pub static LOCATION: &'static str = "Location";
    pub static P3P: &'static str = "P3P";
    pub static PRAGMA: &'static str = "Pragma";
    pub static PROXY_AUTHENTICATE: &'static str = "Proxy-Authenticate";
    pub static REFRESH: &'static str = "Refresh";
    pub static RETRY_AFTER: &'static str = "Retry-After";
    pub static SERVER: &'static str = "Server";
    pub static SET_COOKIE: &'static str = "Set-Cookie";
    pub static STATUS: &'static str = "Status";
    pub static STRICT_TRANSPORT_SECURITY: &'static str = "Strict-Transport-Security";
    pub static TRAILER: &'static str = "Trailer";
    pub static TRANSFER_ENCODING: &'static str = "Transfer-Encoding";
    pub static VARY: &'static str = "Vary";
    pub static VIA: &'static str = "Via";
    pub static WARNING: &'static str = "Warning";
    pub static WWW_AUTHENTICATE: &'static str = "WWW-Authenticate";
}

/// This is a simple type alias for a map of headers
pub type Headers = HashMap<~str,~str>;
