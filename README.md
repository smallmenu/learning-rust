# learning rust

## cargo

### 新建项目或者库

```shell
# A binary
cargo new foo

# A library
cargo new --lib bar
```

### 运行 （多个二进制文件 --bin）

```shell
cargo run

cargo run --bin my_other_bin
```

### 检查

快速的检查一下代码能否编译通过

```shell
cargo check
```

### 构建

```shell
cargo build 

cargo build --release
```

### Rust 升级或运行多个版本

```shell
rustup install 1.93.0

```

## Cargo.toml

### 优化级别

更高的优化级别可能会加快运行时代码的运行速度，但会增加编译时间。

0 是未进行任何优化，3 是所有优化，下面是默认值：

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

### strip

是否移除符号或调试信息

strip = true 等效于 strip = "symbols"

### lto

thin 训练时间大大缩短，同时仍能获得与 "fat" LTO训练类似的性能提升。

lto = "thin"

### panic

发生恐慌时终止该进程

panic = "abort"

### codegen-units

更多的代码生成单元允许并行处理 crate 的更多部分，从而可能缩短编译时间，但可能会生成速度较慢的代码。

dev 默认是：
codegen-units = 256

release 默认是：
codegen-units = 16

## VSCODE 插件

CodeLLDB
rust-analyzer
Even Better TOML
Code Runner

## 常见错误

1. error: failed to install component: 'rust-src', detected conflict: 'lib/rustlib/src/rust/Cargo.lock'

可能升级的时候冲突了，卸载重新安装

```shell
rustup toolchain uninstall stable
rustup component add rust-src
```
