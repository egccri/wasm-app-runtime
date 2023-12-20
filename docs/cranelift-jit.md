# Cranelift

#### ISLE(DSL like ReScript)

https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/isle/docs/language-reference.md

#### SSA(static single assignment form)

IR的中间语言的一个特性，即一个变量只被赋值一次，在原始的IR中，已存在的变量可被分割成许多不同的版本，在许多教科书当中通常会将旧的变量名称加上一个下标
而成为新的变量名称，以至于标明每个变量及其不同版本。在SSA，UD链（use-define chain，赋值代表define，使用变量代表use）是非常明确，而且每个仅包含单一元素。

#### 寄存器分配

常见的寄存器分配方式有以下几种：

+ Basic: 一种增量的寄存器分配方法。
+ Fast: 调试构建的默认方法。
+ Greedy: 默认分配器。这是Basic分配器的一个高度调优的实现，它包含了全局活动范围分割。
+ PBQP: 基于分区布尔二次划分的寄存器分配器。

#### Jit

只需使用 Windows VirtualProtect() API 函数即可完成。它更改虚拟内存页面属性。从 PAGE_READWRITE 中，JIT 编译器可以将机器代码写入 PAGE_EXECUTE_READ 中，
以便可以执行它。执行此操作不需要特殊权限，因为该页面由同时运行 JIT 编译器的进程拥有.

有没有人知道cranelift把wasm编译成机器码，机器码是怎么执行的，rust怎么让渡切换执行机器码的？

https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/jit/src/memory.rs#L177