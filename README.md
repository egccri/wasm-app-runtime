# egccri-runtime
An experimental wasm app runtime.

Wasi include in wasm-component(contains interface type), define in wit file, then use linker link to the 
wasmtime component.

#### Features

+ async
+ multi invokers

#### How to Use

```shell
# run server
egccri run 0.0.0.0:9999

# install app use client
egccri-client -s 127.0.0.1:9999 install grpc-service 0.1.0
```

#### Other

Publish and run guest from registry.

```shell
cargo install --git https://github.com/bytecodealliance/registry
```

Use cargo component impl guest.
```shell
cargo component build
```

#### Example
How to add a pub/sub support with rust and wasmtime?

+ consider use wasi support io function
+ design a `pub/sub` protocol base tcp
+ use a wit file to describe `pub/sub` interface
+ use `wit-bindgen` generate host/guest code
+ impl `pub/sub` base host generate code with wasmtime
+ link `pub/sub` component to wasmtime
+ give a sdk use `wit-bindgen`(extend 'C')
+ write a guest crate to use `pub/sub` sdk
+ support run wasm with cmd or invoker

Flow upcoming wasi-messaging version and impl Host function base tcp.

#### Q&A

Wasi preview1 different with preview2?
Wasi standard changed, how rust compiler also need change? watch lib/sys/wasi.
Wasi func store where in wasm binary? import section.