# Cranelift

#### ISLE(DSL like ReScript)

https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/isle/docs/language-reference.md



#### Jit

只需使用 Windows VirtualProtect() API 函数即可完成。它更改虚拟内存页面属性。从 PAGE_READWRITE 中，JIT 编译器可以将机器代码写入 PAGE_EXECUTE_READ 中，
以便可以执行它。执行此操作不需要特殊权限，因为该页面由同时运行 JIT 编译器的进程拥有.