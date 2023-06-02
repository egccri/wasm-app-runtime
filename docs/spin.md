# Spin

在Spin中，每次trigger触发，比如一个http请求或者一个redis消息，都会新建一个Instance和Store，
而状态的修改是通过主机函数提供的，通过在外界提供key-value、mysql等提供，还包括WASI标准

重要！！！
在trigger触发处理完成后，需要手动删除，因为Store不会自动释放。
```rust
let Some(msg) => drop(self.handle(msg).await);
```

inbound 表示 外界 -> trigger -> wasm
outbound 表示 wasm -> 外界