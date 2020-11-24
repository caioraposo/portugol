<p>
    <img src="docs/img/logo.png" height="200px">

</p>

# Yet Another Portugol Compiler, Written in Rust.

This is an atempt to write a compiler for the **Portugol** language. It is actually a transpiler from **Portugol** to **Python**.

# How to use it

Install [Rust](https://www.rust-lang.org/), clone this repo and build it with cargo.
```
cargo build --release
```
It expects an input file with the Portugol source code, and outputs the Python code to stdout.
```
cargo run --release -- <filename>
```

Based on the compiler from [Writing a Compiler in Go](compilerbook.com).
Thank you, Shuhei Kagawa.
see: https://shuheikagawa.com/blog/2019/10/06/interpreter-and-compiler-in-rust/
