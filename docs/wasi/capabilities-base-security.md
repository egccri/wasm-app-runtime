# capabilities base security

### 基于功能的安全模型和分层特权模型有什么区别

基于功能的安全模型主要解决两个问题

+ 代理的权限问题
+ 沙箱的最小权限问题

传统操作系统的基于分层的权限有哪些问题？

+ 环境权威，用户拥有所在环境的全部权限
+ 复杂的权限检测

example
```text

 Userspace           Name                     Current proc uid 
                       |                             ?
 Kernel               Object   ->   Perms    ->    Allow?

```

##### 不同

如上例所示，分层权限需要在最后的时候去检测是否有权限，而基于能力的模式下，item自身可以知道自己有哪些权限，并且当时就可以验证自己

### 基于的功能特权模型item长什么样

`[path: authority]` 加密不可篡改

### 为什么需要cap

+ 相比于VM等的强隔离，更轻量化，更灵活
+ 多个wasm之间还可以基于capabilities共享能力

### Cap-std

`cap-std`实现了环境权威系统和基于功能的安全模型之间的桥梁，cap-std底层拥有所有的特权环境的功能，然后提供代理接口，
供上层wasm沙箱调用，如果wasm想要某个路径的某个权限，`cap-std`会尝试通过特权环境打开，并反馈给wasm。