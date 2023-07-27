# egccri-runtime
An experimental runtime.

Wasi include in wasm-component(contains interface type), define in wit file, then use linker link to the 
wasmtime module.

How to add a pub/sub support with rust and wasmtime?

+ consider use wasi support io function
+ use a wit file to describe `pub/sub` interface
+ design a `pub/sub` protocol base tcp
+ use `wit-bindgen` generate host/guest code 
+ impl `pub/sub` base host generate code for wasmtime
+ link `pub/sub` component to wasmtime
+ give a sdk use `wit-bindgen`(extend 'C')
+ write a guest crate to use `pub/sub` sdk
+ support run wasm with cmd

#### Client

```shell
# run server
egccri run 0.0.0.0:9999

# install app use client
egccri-client -s 127.0.0.1:9999 install grpc-service 0.1.0
```

#### Next

Publish and run guest from registry.

```shell
cargo install --git https://github.com/bytecodealliance/registry
```

Use cargo component impl guest.
```shell
cargo component build
```

Flow upcoming wasi-messaging version and impl Host function base tcp.

#### Q&A

Wasi preview1 different with preview2?
Wasi standard changed, how rust compiler also need change? watch lib/sys/wasi.
Wasi func store where in wasm binary? import section.