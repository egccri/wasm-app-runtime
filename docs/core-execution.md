# WebAssembly Core Execution

#### 执行流程

module里定义了模块如何初始化的代码

https://webassembly.github.io/spec/core/exec/modules.html#exec-instantiation

module 文件像一个代码， 通过 instantiating 生成 Instances， Instances 是运行时的内存里的代码快照，但以及完成函数之类的绑定，具体参考编译原理。