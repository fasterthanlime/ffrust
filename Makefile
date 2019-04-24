
ifndef FFMPEG_DIR
$(error FFMPEG_DIR must be set)
endif

.PHONY: all

all:
	bindgen in/in.h --rustified-enum '*' --no-prepend-enum-name --with-derive-eq -- -I ${FFMPEG_DIR}/include > src/bindings.rs

