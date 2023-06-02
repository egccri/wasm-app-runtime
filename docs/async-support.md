# 运行时异步

##### 首先要了解wasm在哪个线程执行，也就说我在主机环境中的当前线程调用call wasm函数，wasm函数在哪个线程执行?

Wasmtime不会在内部生成任何任务或线程，所有异步都发生在调用Wasmtime的原始异步任务中。也就是说将wasm看作一个整体的方法，
他是一个Future，调用他或者被他调用都在主机当前线程中，_但wasm执行是在哪个线程？_

##### 运行时异步分为两个方向：`host call wasm`、 `wasm call host`，具体细节是什么？

细节是`wasm call host`时，host函数可能pending，wasmtime这个时候会通过切换stack来暂停wasm的执行，并传递pending 到host，
所以从`host call wasm`时也会pending。

##### Wasmtime异步实现中，epoch配置有什么作用?

参考`refs[3]`。


##### Wasi当前为preview2，接下来的preview3主要实现异步支持，Wasi的异步支持和runtime的异步支持如何理解？

Wasmtime异步支持是host与wasm互相调用的异步，但wasm内部是作为一个整体的block来执行，而Wasi异步提案是为了支持wasm内部的异步支持。
参考`refs[4]`, rust编译器支持wasi提案。

#### refs

1. [explain wasmtime async support](https://github.com/bytecodealliance/wasmtime/issues/3638)
2. [wasm stack switch purpose](https://github.com/WebAssembly/stack-switching)
3. [wasmtime epoch](https://docs.rs/wasmtime/latest/wasmtime/struct.Config.html#method.epoch_interruption)
4. [wasm32-wasi-preview2](https://github.com/rust-lang/compiler-team/issues/594)
