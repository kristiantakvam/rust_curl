RUSTOPTS:=

compile:
	rust build $(RUSTOPTS) rust_curl.rc

test:
	rust test rust_curl.rc

clean:
	rm -f *.so rust_curl rust_curltest~

# vim: ts=8 sw=8 tw=0 noet :    
