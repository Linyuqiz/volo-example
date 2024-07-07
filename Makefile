
.PHONY: server

server: micro
	@cd server && cargo run
