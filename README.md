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
| | | | | | [1](#1)|[2](#2)|
|[3](#3)|4|5|6|7|8|9|
|10|11|12|13|14|15|16|
|17|18|19|20|21|22|23|
|24|25|26|27|28|29|30|
|31|

## Pages
> It is used to describe some learning details(blogs).  
## (Step2)Daily Summary



<span id="1"></span>
### [↑](#TOC)Day 1 (8.3)
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



<span id="2"></span>
### [↑](#TOC)Day 2 (8.4)

#### 阅读毕业设计《zCore 操作系统内核的设计与实现》


#### 现有的zCore 文档主要从这些方面展开描述（以及代码索引）

1. 内核对象
1.1. 初识内核对象   
1.2. 对象管理器：Process 对象       <zircon-object\src\task\process.rs>  job/process/thread  
1.3. 对象传送器：Channel 对象       <zircon-object\src\ipc\channel.rs>  
2. 任务管理                        <zircon-object\src\task>  
2.1. Zircon 任务管理体系            <>  
2.2. 硬件抽象层与``异步运行时``      <kernel_hal(bare)> async 《zCore 操作系统内核的设计与实现》中有相关描述  
2.3. 线程管理：Thread 对象          <zircon-object\src\task\thread.rs>std::thread(8.4日)  
2.4. 进程管理：Process 与 Job 对象  <zircon-object\src\task\job.rs>  <zircon-object\src\task\job_policy.rs>  
3. 内存管理  
3.1. Zircon 内存管理模型  
3.2. 物理内存：VMO 对象             <zircon-object\src\vm\vmo\physical.rs>   
3.3. 虚拟内存：VMAR 对象            <zircon-object\src\vm\vmar.rs>  
4. 用户程序
4.1. Zircon 用户程序                
4.2. 加载 ELF 文件                  <zircon-object\src\util\elf_loader.rs>  
4.3. 上下文切换                     
4.4. 系统调用                       <zircon-syscall\src>
#### zCore项目整理架构
![](zCore\pics\1.png)

#### 个人打算文档的编写依据如下线路编写?(LibOS)  
Linux/MacOS  ->  
kernel-hal-unix ->   
kernel-hal ->  
zircon-object/syscall/loader  



#### 文档其他没有被包含的内容  
+ boot相关？(8.5放到后面写)  
+ linux-loader/busybox(8.5助教说暂时可以不用看)  
+ baremental <zCore\src\arch\x86_64\linker.ld>section之间的4K对齐描述  
#### zCore 整体设计（Fuchsia）  
[前期调研](http://os.cs.tsinghua.edu.cn/oscourse/OsTrain2019/g1)  
zCore是微内核结构  
+ zCore立项背景  
支持各种linux System Call  
+ 测试集  
功能测试: Core Tests  
BENCHMARK:  QEMU-KVM 1 CPU测试  
            cargo bench  
+ 类似make weak  
(all)user mode  
#### async  
无栈协程，协作式调度  
C++/C#/python/JS  
BLOGOS:paper!  
tokio/async-std  



---







<span id="3"></span>
### [↑](#TOC)Day 3 (8.5)

#### 

---