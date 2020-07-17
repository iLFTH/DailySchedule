> 以下内容通过参考如下实验指导完成
https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-2/guide/part-1.html

#### 内核中动态内存分配,典型的应用场景有：

+ Box<T> ，你可以理解为它和 malloc 有着相同的功能；
+ 引用计数 Rc<T>，原子引用计数 Arc<T>，主要用于在引用计数清零，即某对象不再被引用时，对该对象进行自动回收；
+ 一些 Rust std 标准库中的数据结构，如 Vec 和 HashMap 等。

#### 实现Trait GlobalAlloc

```rust
unsafe fn alloc(&self, layout: Layout) -> *mut u8;
unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);
```
这里面的 Layout 它有两个字段：
+ size 表示要分配的字节数
+ align 则表示分配的虚拟地址的最小对齐要求，即分配的地址要求是 align 的倍数。这里的 align 必须是 2 的幂次。  

也就表示，我们的需求是分配一块连续的、大小至少为 size 字节的虚拟内存，且对齐要求为 align 。


#### 伙伴系统（Buddy System）
##### 开辟8MB的堆 (os/src/memory/config.rs)
```rust
/// 操作系统动态分配内存所用的堆大小（8M）
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;
```