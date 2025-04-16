## LZAV - Fast Data Compression Algorithm

This is the LZAV implementation for use in Rust. It compiles the original C/C++ code using the pipeline clang -> llvm-as -> llc -> ar into a static library and links it to this Rust library, thereby providing bindings for working with the original methods in Rust.

In addition, `safeCompress(input: &[u8]) -> Option <Vec<u8> >` is available for simple and safe compression, and `pub fn safeDecompress(inputData: &[u8]) -> Option< Vec<u8> >` for safe decompression of data with unknown size.

More detailed documentation is available for each method. Original [repository](https://github.com/avaneev/lzav).
