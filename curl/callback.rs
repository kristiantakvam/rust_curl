use libc::{size_t};
use std::mem;

pub type CurlCallbackType<D, U> = extern "C" fn (data: *D, size: size_t, nmemb: size_t, user_data: *U) -> size_t;

/// Generic trait to represent a CURL callback data/function pair. 
pub trait CurlCallback<D, U> {
    fn curl_get_userdata<'a>(&'a self) -> &'a U;
    fn curl_get_callback(&self) -> CurlCallbackType<D, U>;
}

/// Simple buffer for data retrieved by curl
pub struct SimpleCurlByteBuffer {
    pub data: Vec<u8>
}

impl SimpleCurlByteBuffer {
    pub fn new() -> SimpleCurlByteBuffer {
        SimpleCurlByteBuffer { data: vec![] }
    }
}

/// Callback implementation for a byte buffer managed by SimpleCurlByteBuffer.
/// Writing is done by expanding the data buffer whenever the curl library calls the rust callback.
impl CurlCallback<u8, Vec<u8>> for SimpleCurlByteBuffer {
    fn curl_get_userdata<'a>(&'a self) -> &'a Vec<u8> {
        &'a self.data
    }

    fn curl_get_callback(&self) -> CurlCallbackType<u8, Vec<u8>> {
        unsafe {
            mem::transmute(c_curl_write_buf_fn)
        }
    }
}

/// Write callback called by libcurl when it receives more data
/// # Arguments
/// * `data` - the data received from this call
/// * `size` - the size each chunk received
/// * `nmemb` - the number of chunks
/// * `user_data` - pointer to user_data returned by the CurlCallback.curl_get_userdata fn
/// # Safety Notes
/// the size of the data received is (size * nmemb), and in this case
/// you should set user_data to be a reference to a ~[u8], although
/// you can write such a function yourself that has different user data
pub extern "C" fn c_curl_write_buf_fn (data: *u8, size: size_t, nmemb: size_t, user_data: *())
    -> size_t {
    use std::vec::raw::from_buf;

    let s: &mut Vec<u8> = unsafe { mem::transmute(user_data) };
    let new_data = unsafe { from_buf(data, (size * nmemb) as uint) };
    s.push_all_move(new_data);
    size * nmemb
}
