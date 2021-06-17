# Linking Rust code into NodeJS example

This project is a mini project to use it as an example for using [Rust](https://www.rust-lang.org) to build a native module to be used on NodeJS.

The main branch should be empty and you should navigate through the branches for the different methods that I have found over the web.

## References

This method implements a native module using N-API ([Node-API](https://nodejs.org/api/n-api.html)) with [nodejs-sys](https://github.com/elmarx/nodejs-sys) crate to make it easier to use the N-API.

Also, this is only the implementations of [this article](https://blog.logrocket.com/rust-and-node-js-a-match-made-in-heaven/) from LogRocket with some minor changes to test each example given on the article.

## Dependencies

First, you need to have the dependencies installed:

* NodeJs
* Rust
* Cargo
* llvm
* libclang-dev
* clang

(For Debian/Ubuntu, with a simple `apt install llvm libclang-dev clang` should be enough).

## Building Rust module

To build the native module it's simple with cargo, just run `cargo build` (or `cargo build --release` for optimized code).
Then you need to copy the compiled code to the root of project, the compiled should be inside of `target/debug` and should be names `librust_addon.so` (if you builded with `--release` it hsould be inside `target/release`).

## Running project

I divided each module into function and passing the "module" name into via cli.

Just run `npm start {module}` to run the specific module to check it out.
