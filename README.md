# Daily Schedule/Summary-2
---
## Links
+ [OS Tutorial Summer of Code](https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code)
+ [Daily Schedule ++](https://github.com/rcore-os/rCore-Tutorial/issues/18)
<!-- + [Web](http://www.nuanyun.cloud) -->
<span id="TOC"></span>  

<!-- ## Tags

- [RISC-V](http://www.nuanyun.cloud/?tag=riscv) 
- [Rust](http://www.nuanyun.cloud/?tag=rust)
- [Qemu](http://www.nuanyun.cloud/?tag=qemu)
- [LLVM](http://www.nuanyun.cloud/?tag=)  -->

## TOC

 *2020 八月*                

| Mon                    | Tues                   | Wed                    | Thur                   | Fri                    | Sat                    | Sun                    |
|------------------------|------------------------|------------------------|------------------------|------------------------|------------------------|------------------------|
| | | | | | [1](#0)|2|
|3|4|5|6|7|8|9|
|10|11|12|13|14|15|16|
|17|18|19|20|21|22|23|
|24|25|26|27|28|29|30|
|31|

## Pages
> It is used to describe some learning details(blogs).  
## (Step2)Daily Summary



<span id="0"></span>
### [↑](#TOC)Day 0 (8.3)
Todo:

#### zCore 的文档与单元测试完善
* 项目标题
zCore 的文档与单元测试完善

* 项目描述
zCore 是用 Rust 重新实现的 Zircon 微内核。目前我们已经按照官方文档的描述，实现了诸多 Zircon 内核对象，支持运行 shell 等基础程序。然而，随着后期开发进程的加快，大量代码缺少文档描述，并且没有实现单元测试，只能在 QEMU 中运行用户程序以检验代码正确性。本项目的目标是完善 zCore 的文档及单元测试，使其成为一个高质量的 Rust 社区项目。

* 项目产出
补充完善 zCore 各模块的代码文档。

> 1. 目标是通过 #![deny(missing_docs)] 编译，并且能够让开发者通过阅读文档，快速理解 zCore 的代码结构和各部分功能。

> 2. 参考 Fuchsia 官方文档及测试代码，为 zCore 中的内核对象补充单元测试。目标让 zircon-object 模块的测试覆盖率提高到 90% 以上。
> 3. （可选）在 CI 中支持运行集成测试。 目标是最大化整体的测试覆盖率。

> 4. （可选）在zCore中添加zircon/linux的syscall。 目标：完善添加zCore内核功能，让zCore通过更多的zircon tests(基于zircon的coretest)或Linux tests（基于musl libc的libc test）或相关应用

相关的开源软件仓库列表：

https://github.com/rcore-os/zCore （zCore 仓库）
https://rcore-os.github.io/zCore/zircon_object （zCore 代码文档）
https://fuchsia.dev/fuchsia-src/reference （Zircon 官方文档）

相关

---



<span id="0"></span>
### [↑](#TOC)Day 1 (8.4)
Todo:

#### 阅读毕业设计《zCore 操作系统内核的设计与实现》
了解ZCore整体加载

zCore启动流程：介绍

---
