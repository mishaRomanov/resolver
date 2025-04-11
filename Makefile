build: 
	sudo cargo build

# usage: make run ipkind=4 domain=google.com
run:
	target/debug/resolver -i $(ipkind) -d $(domain)
