RUSTOPTS:=

compile:
	rust build $(RUSTOPTS) rust_curl.rc

test:
	rust test rust_curl.rc

# vim: ts=8 sw=8 tw=0 noet :    
