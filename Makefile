DS_VERSION="0.6.1"
ARCH="amd64"
PLATFORM="cpu"
OS="linux"

all: $(TARGET) clean build 

clean:
	@cargo clean && rm -rf deps/

setup:
	@mkdir -p deps/

deps: setup
	@wget -q -nc -O deps/client.tar.xz \
		https://github.com/mozilla/DeepSpeech/releases/download/v$(DS_VERSION)/native_client.$(ARCH).$(PLATFORM).$(OS).tar.xz; \
	tar -xf deps/client.tar.xz -C deps/

build: deps
	@LIBRARY_PATH=$(shell pwd)/deps cargo build

.PHONY: clean setup deps build
