# Cranelift

#### ISLE(DSL like ReScript)

https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/isle/docs/language-reference.md

#### SSA



#### 寄存器分配

常见的寄存器分配方式有以下几种：

+ Basic: 一种增量的寄存器分配方法。
+ Fast: 调试构建的默认方法。
+ Greedy: 默认分配器。这是Basic分配器的一个高度调优的实现，它包含了全局活动范围分割。
+ PBQP: 基于分区布尔二次划分的寄存器分配器。

#### Jit

只需使用 Windows VirtualProtect() API 函数即可完成。它更改虚拟内存页面属性。从 PAGE_READWRITE 中，JIT 编译器可以将机器代码写入 PAGE_EXECUTE_READ 中，
以便可以执行它。执行此操作不需要特殊权限，因为该页面由同时运行 JIT 编译器的进程拥有.