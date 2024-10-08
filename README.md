# Rust WASM Playground

A dummy project for learning to use Rust on the web.

## Building

Build with the wasm-pack crate

```sh
wasm-pack build --target web [--release]
```

The `web` target is required because of how WASM is loaded. Compare the results of `pkg/tinywasm_playground.js` with the default target and the web target. Further docs [here](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html)

## Hosting

Serve with your favorite webserver

```sh
# I like the idea of using lighttpd as a production server
lighttpd -D -f ./lighttpd.conf

# Python's test server works well for development
python3 -m http.server

# Or the Rust miniserve crate
miniserve --index index.html
```
