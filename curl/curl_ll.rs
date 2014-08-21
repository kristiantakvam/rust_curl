use libc::{c_long, c_int, c_char, c_void};

use curl::code::CURLcode;
use curl::opt::CURLoption;

static CURLINFO_STRING: int = 0x100000;
static CURLINFO_LONG: int = 0x200000;
static CURLINFO_DOUBLE: int = 0x300000;
static CURLINFO_SLIST: int = 0x400000;
static CURLINFO_MASK: int = 0x0fffff;
static CURLINFO_TYPEMASK: int = 0xf00000;

pub enum CURLINFO {
  CURLINFO_NONE, /* first, never use this */
  CURLINFO_EFFECTIVE_URL    = CURLINFO_STRING + 1,
  CURLINFO_RESPONSE_CODE    = CURLINFO_LONG   + 2,
  CURLINFO_TOTAL_TIME       = CURLINFO_DOUBLE + 3,
  CURLINFO_NAMELOOKUP_TIME  = CURLINFO_DOUBLE + 4,
  CURLINFO_CONNECT_TIME     = CURLINFO_DOUBLE + 5,
  CURLINFO_PRETRANSFER_TIME = CURLINFO_DOUBLE + 6,
  CURLINFO_SIZE_UPLOAD      = CURLINFO_DOUBLE + 7,
  CURLINFO_SIZE_DOWNLOAD    = CURLINFO_DOUBLE + 8,
  CURLINFO_SPEED_DOWNLOAD   = CURLINFO_DOUBLE + 9,
  CURLINFO_SPEED_UPLOAD     = CURLINFO_DOUBLE + 10,
  CURLINFO_HEADER_SIZE      = CURLINFO_LONG   + 11,
  CURLINFO_REQUEST_SIZE     = CURLINFO_LONG   + 12,
  CURLINFO_SSL_VERIFYRESULT = CURLINFO_LONG   + 13,
  CURLINFO_FILETIME         = CURLINFO_LONG   + 14,
  CURLINFO_CONTENT_LENGTH_DOWNLOAD   = CURLINFO_DOUBLE + 15,
  CURLINFO_CONTENT_LENGTH_UPLOAD     = CURLINFO_DOUBLE + 16,
  CURLINFO_STARTTRANSFER_TIME = CURLINFO_DOUBLE + 17,
  CURLINFO_CONTENT_TYPE     = CURLINFO_STRING + 18,
  CURLINFO_REDIRECT_TIME    = CURLINFO_DOUBLE + 19,
  CURLINFO_REDIRECT_COUNT   = CURLINFO_LONG   + 20,
  CURLINFO_PRIVATE          = CURLINFO_STRING + 21,
  CURLINFO_HTTP_CONNECTCODE = CURLINFO_LONG   + 22,
  CURLINFO_HTTPAUTH_AVAIL   = CURLINFO_LONG   + 23,
  CURLINFO_PROXYAUTH_AVAIL  = CURLINFO_LONG   + 24,
  CURLINFO_OS_ERRNO         = CURLINFO_LONG   + 25,
  CURLINFO_NUM_CONNECTS     = CURLINFO_LONG   + 26,
  CURLINFO_SSL_ENGINES      = CURLINFO_SLIST  + 27,
  CURLINFO_COOKIELIST       = CURLINFO_SLIST  + 28,
  CURLINFO_LASTSOCKET       = CURLINFO_LONG   + 29,
  CURLINFO_FTP_ENTRY_PATH   = CURLINFO_STRING + 30,
  CURLINFO_REDIRECT_URL     = CURLINFO_STRING + 31,
  CURLINFO_PRIMARY_IP       = CURLINFO_STRING + 32,
  CURLINFO_APPCONNECT_TIME  = CURLINFO_DOUBLE + 33,
  CURLINFO_CERTINFO         = CURLINFO_SLIST  + 34,
  CURLINFO_CONDITION_UNMET  = CURLINFO_LONG   + 35,
  CURLINFO_RTSP_SESSION_ID  = CURLINFO_STRING + 36,
  CURLINFO_RTSP_CLIENT_CSEQ = CURLINFO_LONG   + 37,
  CURLINFO_RTSP_SERVER_CSEQ = CURLINFO_LONG   + 38,
  CURLINFO_RTSP_CSEQ_RECV   = CURLINFO_LONG   + 39,
  CURLINFO_PRIMARY_PORT     = CURLINFO_LONG   + 40,
  CURLINFO_LOCAL_IP         = CURLINFO_STRING + 41,
  CURLINFO_LOCAL_PORT       = CURLINFO_LONG   + 42,
  /* Fill in new entries below here! */

  CURLINFO_LASTONE          = 42
}

/// This is a the curl_slist structure, a list of c strings
///
/// This is a simple singly-linked list of c_strings
///     that is obviously an unsafe structure which should
///     be used properly and cautiously
pub struct curl_slist {
    data: *c_char,
    next: *curl_slist
}

pub type CURL = c_void;

#[link(name = "curl")]
extern {
    pub fn curl_global_init(flags: c_long) -> CURLcode;

    /* Easy interface */
    pub fn curl_easy_cleanup(handle: *CURL) -> c_void;
    pub fn curl_easy_duphandle(handle: *CURL) -> *CURL;
    pub fn curl_easy_escape(curl: *CURL, url: *c_char, length: c_int) -> *c_char;
    pub fn curl_easy_getinfo(handle: *CURL, info: CURLINFO, arg: *c_void) -> CURLcode;
    pub fn curl_easy_init() -> *CURL;
    pub fn curl_easy_perform(handle: *CURL) -> CURLcode;
    // Skipping curl_easy_recv
    pub fn curl_easy_reset(handle: *CURL) -> c_void;
    // Skipping curl_easy_send
    pub fn curl_easy_setopt(handle: *CURL, opt: CURLoption, val: *c_void) -> CURLcode;
    pub fn curl_easy_strerror(err: CURLcode) -> *c_char;
    pub fn curl_easy_unescape(curl: *CURL, url: *c_char, inlength: c_int, outlength: *c_int) -> *c_char;
    
    /* Utility */
    pub fn curl_free(ptr: *c_char) -> c_void;

    pub fn curl_slist_append(list: *curl_slist, s: *c_char) -> *curl_slist;
    pub fn curl_slist_free_all(list: *curl_slist) -> c_void;
}
