## WASM cookies for Rust

*Allows to manage cookies in and outside of the browser with Rust and WebAssembly.*

This crate use `wasm-bindgen` and `web-sys`. See the [Documentation](https://docs.rs/wasm-cookies). But it can also be used without it outside of a browser, and these to dependencies will not be imported if the target is not "wasm32-unknown-unknown".

To contribute, see [Contributing](CONTRIBUTING.md).