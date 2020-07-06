# Daily Schedule/Summary
---
## Links
+ [OS Tutorial Summer of Code](https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code)
+ [Daily Schedule ++](https://github.com/rcore-os/rCore-Tutorial/issues/18)
+ [iLFTH's Web](http://www.nuanyun.cloud)
<span id="TOC"></span>  

## Tags

- [RISC-V](http://www.nuanyun.cloud/?tag=riscv) 
- [Rust](http://www.nuanyun.cloud/?tag=rust)
- [Qemu](http://www.nuanyun.cloud/?tag=qemu)
- [LLVM](http://www.nuanyun.cloud/?tag=) 

## TOC



 *2020七月*                

| Mon                    | Tues                   | Wed                    | Thur                   | Fri                    | Sat                    | Sun                    |
|------------------------|------------------------|------------------------|------------------------|------------------------|------------------------|------------------------|
|     |    | 1  | 2  | [3](#0)  | [4](#1)     | [5](#2)     |
| 6   | 7  | 8  | 9  | 10 | 11 | 12 |
| 13  | 14 | 15 | 16 | 17 | 18 | 19 |
| 20  | 21 | 22 | 23 | 24 | 25 | 26 |
| 27  | 28 | 29 | 30 | 31 |                        |                        |

 *2020 八月*                

| Mon                    | Tues                   | Wed                    | Thur                   | Fri                    | Sat                    | Sun                    |
|------------------------|------------------------|------------------------|------------------------|------------------------|------------------------|------------------------|
| | | | | | 1|2|
|3|4|5|6|7|8|9|
|10|11|12|13|14|15|16|
|17|18|19|20|21|22|23|
|24|25|26|27|28|29|30|
|31|

## Pages
> It is used to describe some learning details(blogs).


## Daily Summary

<span id="1"></span>
### [↑](#TOC)Day 1 (7.3)
学习办公环境整理，搬寝室
建立此repo

---
<span id="2"></span>
### [↑](#TOC)Day 2 (7.4)
#### 1. rust编程语言环境配置
参考阅读《Rust 程序设计语言》简体中文版，在linux/macos上安装rustup  
latest update on 2020-06-18, rust version 1.44.1  
能够运行一些样例程序
#### 2. 理解常见rust编程概念
+ 不可变变量（与常量的区别）
+ 数据类型，及类型注释方法
  - rust术语：Rust panic 它用于程序因错误而退出的情况。  
  - 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
+ 变量/常量/函数名命名规范
+ 函数定义方式（传参/返回值的写法）
+ 函数定义内：表达式和语句的区别
+ 了解rust的控制流写法（if/loop/while/for）
  - 同时归纳与C语言写法的一些差异


---
<span id="3"></span>
### [↑](#TOC)Day 3 (7.5)    

#### 1. 阅读《通过例子学 Rust》
+ [文档注释方法](https://rustwiki.org/zh-CN/rust-by-example/meta/doc.html)
+ 格式化输出
+ 原生类型/自定义类型
+ 了解rust的剩余控制流写法
    - match
    - if/while let
+ 学习rust 链表写法([示例](RUST/Toy_Srcs/List.rs))基于enum
+ 类型系统
    - 可以用 type 语句给已有的类型取个新的名字。类型的名字必须遵循驼峰命名法（像是 CamelCase 这样），否则编译器将给出错误。示例如下：
        ```rust
        type NanoSecond = u64;
        type Inch = u64;
        ```
    - 类型转换：use std::convert::From;  
        ```rust
        let my_str = "hello";
        let my_string = String::from(my_str);
        // &str常见形式是字符串字面量 
        // String基于堆创建，是可增长的字符串。
        ```
+ 了解了函数 方法(method),同时类比了C++的类(Class)
+ 了解了函数 [闭包(closure)和捕获（capture）](https://rustwiki.org/zh-CN/rust-by-example/fn/closures/capture.html)  
> 先jump to对所有权的学习（书中的示例代码用到了许多有关所有权的知识，打算在明天学习）
+ [章节后小练习](RUST/Toy_Srcs/)
#### 2. Rust教程
+ https://www.yiibai.com/rust/rust-smart-pointers.html ： Rust智能指针学习
#### 3. 参考之前的个人实验笔记，在linux中重新安装QEMU
Try to **Re**Run [64-bit RISC-V Linux on QEMU](http://www.nuanyun.cloud/?p=430)
应当可为后续实验做准备

#### ... [详细记录情况🔗](RUST/docs/7.5/mark.md)








---
<span id="4"></span>
### [↑](#TOC)Day 4 (7.6)    

重点阅读《Rust 编程之道》
#### 1. 类型系统和所有权（Chapter-3/4/5）
+ 类型安全：  
    + 类型安全的语言可以避免类型间的无效计算
    + 类型安全的语言还可以保证内存安全
    + 类型安全的语言也可以避免语义上的逻辑错误
+ 胖指针与Copy traint的区别


#### 2. 理解错误处理（Chapter-9）
#### 3. 理解Unsafe（Chapter-13）
#### 4. https://github.com/rust-lang/rustlings


#### ... [详细记录情况🔗](RUST/docs/7.6/mark.md)
---
<span id="5"></span>
### [↑](#TOC)Day 5 (7.7)    

#### todo: 
+

---
<span id="6"></span>
### [↑](#TOC)Day 6 (7.8)    

#### todo: 
+ 

---

    > step0 todo list:
    > 通过阅读和练习 《 Rust by Example 》) ，全面梳理一遍 Rust 语法。期间可参考书目：（《Rust权威指南》（即官方 Rust Book 中译本）、《Rust编程之道》第二章 、《深入浅出 Rust》等）。（两天）
    > 重点阅读《Rust 编程之道》 （第三、四、五章，理解类型系统和所有权；第九章，理解错误处理； 第十三章，理解Unsafe Rust） （一天）
    > 完成《Rust 编程之道》第十章的完整示例代码，掌握Cargo和模块系统。（一天）
    > 尝试完成编程小练习。（三天）