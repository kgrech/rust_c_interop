# Rust and C Interoperability [![Build Status](https://travis-ci.com/kgrech/rust_c_interop.svg?branch=main)](https://travis-ci.com/kgrech/rust-algorithms)

The project is a little demo on how to compile and use Rust and C together.
It demos 7 ways to pass strings between rust and C.

Read more on [dev.to](https://dev.to/kgrech/7-ways-to-pass-a-string-between-rust-and-c-4ieb)

## Run
To run debug build with address sanitizer
```
cmake -DCMAKE_BUILD_TYPE=Debug .
make
./rust_c_interop
```
