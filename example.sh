#!/bin/sh

chk() {
	cat /dev/stdin |
		wasmtime \
			run \
			./rs-imghead2mime.wasm

	echo
}

echo jpeg test
printf '\xff\xd8\xff' | chk

echo png test
printf '\x89\x50\x4e\x47\r\n\x1a\n' | chk

echo gif test
printf 'GIF87a' | chk

echo bmp test
printf 'BM' | chk

echo webp test
printf 'RIFF\0\0\0\0WEBP' | chk

echo tiff test
printf 'II*\0' | chk

echo ico test
printf '\0\0\1\0' | chk

echo heic test
printf '\0\0\0\0ftypheic' | chk
