APP_NAME = rustmarks
DESTDIR = /usr/local/bin

build:
	cargo build --release

install: build
	sudo cp target/release/$(APP_NAME) $(DESTDIR)/$(APP_NAME)
