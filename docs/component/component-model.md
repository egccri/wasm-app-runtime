# Component

#### 什么是组件

Component 位于 Core WebAssembly 之上，可以理解为 Core 的外挂，由 Executed 实现。

Component 的承载形式是一个自描述的可执行二进制文件。

#### 组件发展的历史

根据组件的定义，我们知道组件模型主要解决

在Wasmtime中Component是这样定义的：

```rust
#[derive(Clone)]
pub struct Component {
    inner: Arc<ComponentInner>,
}

struct ComponentInner {
    /// Core wasm modules that the component defined internally, indexed by the
    /// compile-time-assigned `ModuleUpvarIndex`.
    static_modules: PrimaryMap<StaticModuleIndex, Module>,

    /// Code-related information such as the compiled artifact, type
    /// information, etc.
    ///
    /// Note that the `Arc` here is used to share this allocation with internal
    /// modules.
    code: Arc<CodeObject>,

    /// Metadata produced during compilation.
    info: CompiledComponentInfo,
}
```

#### 组件的现状