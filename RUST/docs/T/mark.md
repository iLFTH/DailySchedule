# 杂项

## Rust

1. Rust开发时有时使用官方的源太慢，可以考虑更换使用国内中科大的源。
在 $HOME/.cargo/config 中添加如下内容：
    ```
    [source.crates-io]
    replace-with = 'ustc'

    [source.ustc]
    registry = "git://mirrors.ustc.edu.cn/crates.io-index"
    ```


2. [rust 数组和向量 Arrays and Vectors](https://github.com/nrc/r4cppp/blob/master/arrays.md)

3. 格式化Rust代码：Cargo 扩展rustfmt
    ```shell
    $ cargo fmt
    ```
## Risc-V

