# Table Of Contents
> This table has some ~~not implement features yet~~ and some _not stable implementation_.

+ Wasm
    - Core(Wasm设计目标，编译原理，堆栈机，抽象语法树)
        - 堆栈机器运行原理
    - Structure
        - WIP
            + function-references
    - Binary Format, Encode
        - Rust编译到Wasm的过程
        - Wat
    - Decode, Validation
        - Wasmtime
        - 验证过程中的安全认证，检测import，cap-std
    - Execution
        - [执行过程](core-execution.md)
        - [Wasmtime执行过程](wasmtime-core-execution.md)
    

+ _Component Model_
    - Summary(history, define, module link, interface type, abi)
        - [summary](component-model.md)
        - [abi](canonical-abi.md)
    - Types
        - Support a type in Wasmtime like resource
    - Binary Format
    - Wit
    - Wasmtime Component Support
    - Application
        - Host-Guest
        - Wasm Compose
            - use wasm-tool compose components
            - ~~wasm compose in Wasmtime~~

+ Wasi
    - What is wasi, and how impl wasi feature base Component Model
    - Impl a wasi feature in rust
    - Impl a wasi feature in wasitime
    - Wasi preview(标准、异步)
    - Wasmtime support wasi

+ 高级内容
    - WASM二进制格式对比Java字节码（安全性、功能）
    - 编译器AOT，JIT以及Wasmtime的实现
    - WASI-Thread提案

+ Rust工具链
    - 编译(cargo-component)
    - 分发(registry)
    - https://wasmbuilder.app/

