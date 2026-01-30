# learning rust

## cargo

### 新建项目或者库

```
# A binary
cargo new foo

# A library
cargo new --lib bar
```

### 运行 （多个二进制文件 --bin）：

```
cargo run

cargo run --bin my_other_bin
```

### 构建

```
cargo build 

cargo build --release
```


## 常见错误

1. error: failed to install component: 'rust-src', detected conflict: 'lib/rustlib/src/rust/Cargo.lock'

可能升级的时候冲突了，卸载重新安装

```
rustup toolchain uninstall stable
rustup component add rust-src
```

