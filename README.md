# egccri-runtime
An experimental runtime.

Wasi include in wasm-component(contains interface type), define in wit file, then use linker link to the 
wasmtime module.

Wasi preview1 different with preview2? 
Wasi standard changed, how rust compiler also need change?
Wasi func store where in wasm binary?

How to add a pub/sub support with rust and wasmtime?

+ design a `pub/sub` protocol base tcp
+ consider use wasi support io function
+ use a wit file to describe `pub/sub` interface
+ use `wit-bindgen` generate bindgen code 
+ impl `pub/sub` with bindgen code
+ link `pub/sub` component to wasmtime
+ give a sdk use extend 'C' to compile rust to target `wasm32-wasi`
+ write a crate with main to use `pub/sub` sdk
+ support run wasm with cmd