use curl;
use request;
use response;
use http_client;

// this function is an example of the simplest functionality
pub fn example_get() {
	use std::str::from_bytes;
	
	let data_res = curl::get("http://api.4chan.org/pol/threads.json");
	
	match data_res {
		Ok(data) => { println(from_bytes(data)); }
		Err(msg) => { fail!("Error" + msg); }
	};
}
