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
