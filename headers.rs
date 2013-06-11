use std::hashmap::HashMap;

macro_rules! str_const(
    ($name:ident $val:expr) => (
        pub static $name: &'static str = $val;
    )
)

pub mod request {
    pub static ACCEPT: &'static str = "Accept";
    pub static ACCEPT_CHARSET: &'static str = "Accept-Charset";
    pub static ACCEPT_ENCODING: &'static str = "Accept-Encoding";
    str_const!(ACCEPT_LANGUAGE "Accept-Language")
    str_const!(ACCEPT_DATETIME "Accept-Datetime")
    str_const!(AUTHORIZATION "Authorization")
    str_const!(CACHE_CONTROL "Cache-Control")
    str_const!(CONNECTION "Connection")
    str_const!(CONTENT_LENGTH "Content-Length")
    str_const!(CONTENT_MD5 "Content-MD5")
    str_const!(COOKIE "Cookie")
    str_const!(DATE "Date")
    str_const!(EXPECT "Expect")
    str_const!(FROM "From")
    str_const!(HOST "Host")
    str_const!(IF_MATCH "If-Match")
    str_const!(IF_MODIFIED_SINCE "If-Modified-Since")
    str_const!(MAX_FORWARDS "Max-Forwards")
    str_const!(ORIGIN "Origin")
    str_const!(PROXY_AUTHORIZATION "Proxy_Authorization")
    str_const!(RANGE "Range")
    str_const!(REFERRER "Referrer")
    str_const!(TE "TE")
    str_const!(UPGRADE "Upgrade")
    str_const!(USER_AGENT "User-Agent")
    str_const!(VIA "Via")
    str_const!(WARNING "Warning")
}

pub mod response {
    str_const!(ACCESS_CONTROL_ALLOW_ORIGIN "Access-Control-Allow-Origin")
    str_const!(ACCEPT_RANGES "Accept-Ranges")
    str_const!(AGE "Age")
    str_const!(ALLOW "Allow")
    str_const!(CACHE_CONTROL "Cache-Control")
    str_const!(CONNECTION "Connection")
    str_const!(CONTENT_ENCODING "Content-Encoding")
    str_const!(CONTENT_LANGUAGE "Content-Language")
    str_const!(CONTENT_LENGTH "Content-Length")
    str_const!(CONTENT_LOCATION "Content-Location")
    str_const!(CONTENT_MD5 "Content-MD5")
    str_const!(CONTENT_DISPOSITION "Content-Disposition")
    str_const!(CONTENT_RANGE "Content-Range")
    str_const!(CONTENT_TYPE "Conetent-Type")
    str_const!(DATE "Date")
    str_const!(ETAG "ETag")
    str_const!(EXPIRES "Expires")
    str_const!(LAST_MODIFIED "Last-Modified")
    str_const!(LINK "Link")
    str_const!(LOCATION "Location")
    str_const!(P3P "P3P")
    str_const!(PRAGMA "Pragma")
    str_const!(PROXY_AUTHENTICATE "Proxy-Authenticate")
    str_const!(REFRESH "Refresh")
    str_const!(RETRY_AFTER "Retry-After")
    str_const!(SERVER "Server")
    str_const!(SET_COOKIE "Set-Cookie")
    str_const!(STATUS "Status")
    str_const!(STRICT_TRANSPORT_SECURITY "Strict-Transport-Security")
    str_const!(TRAILER "Trailer")
    str_const!(TRANSFER_ENCODING "Transfer-Encoding")
    str_const!(VARY "Vary")
    str_const!(VIA "Via")
    str_const!(WARNING "Warning")
    str_const!(WWW_AUTHENTICATE "WWW-Authenticate")
}

pub type Headers = HashMap<~str,~str>;
