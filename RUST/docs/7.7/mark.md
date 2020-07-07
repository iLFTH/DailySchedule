<span id="5"></span>
### [↑](#TOC)Day 4 (7.7)    

todo：

#### 完成《Rust 编程之道》第十章的完整示例代码，掌握Cargo和模块系统。
> cargo new 命令默认创建的是库文件（生成静态或动态链接库）  
![](pics/1.png)

> 目录结构  
![](pics/2.png)  

> 测试test模块  
![](pics/3.png)

> cargo run的debug和release模式选择（默认debug）  
![](pics/4.png) 

> 调用第三方crate  
![](pics/5.png)

> 添加正则表达式依赖  
![](pics/7.png)
![](pics/6.png)

<span id="lazy_static"></span>
![](pics/9.png)  



RwLock 读写锁它允许多个线程读，单个线程写
Metux 互斥锁只允许单个线程读和写。
> 所以在读数据比较频繁远远大于写数据的情况下 使用 RwLock 读写锁可以给程序带来
更高的并发支持。
---