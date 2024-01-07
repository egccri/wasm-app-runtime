# Registry协议Warg使用

Warg是基于HTTP的Wasm二进制包分发的协议，包含Publish, Fetch, Proof等，提供了可选的后端存储，包含内存和Postgresql。

对于Rust开发者来说，我们可以使用cargo-component或者warg-client来Publish Wasm二进制包，cargo-component在内部集成了warg-client，
当然，我们也可以在自己的项目中集成warg-client。

项目的架构如下：

```console
cargo-component
                  |->      registry(warg-server) 
warg-client
```

Registry整体包含以下crates（crate是rust项目的模块）：
```text
registry: 项目主体，提供warg命令
    api: client和server公用的实体，包含序列化
    client: warg client端主要逻辑
    crypto: Hash加密相关的东西
    protocol: 协议
    server: 服务端
    transparency: 验证相关的功能
```

### wasm上传的生命周期

