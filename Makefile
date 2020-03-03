DS_VERSION=0.6.1
ARCH=amd64
PLATFORM=cpu
ifeq ($(shell uname -s),Darwin)
	OS=mac
else
	OS=linux
endif

all: $(TARGET) clean build 

clean:
	@cargo clean && rm -rf deps/

setup:
	@mkdir -p deps/

models:
	@wget -q -nc -O deps/models.tar.gz \
		https://github.com/mozilla/DeepSpeech/releases/download/v$(DS_VERSION)/deepspeech-$(DS_VERSION)-models.tar.gz; \
		tar -xzf deps/models.tar.gz -C deps/

lib:
	@wget -q -nc -O deps/client.tar.xz \
		https://github.com/mozilla/DeepSpeech/releases/download/v$(DS_VERSION)/native_client.$(ARCH).$(PLATFORM).$(OS).tar.xz; \
	tar -xf deps/client.tar.xz -C deps/

build: setup models lib
	@LIBRARY_PATH=$(shell pwd)/deps cargo build --verbose

test: setup models lib
	@LIBRARY_PATH=$(shell pwd)/deps cargo test --all 

.PHONY: clean setup lib models build test
