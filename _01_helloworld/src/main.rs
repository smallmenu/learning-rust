use std::fmt;

// `Structure` 是一个包含单个 `i32` 的结构体。 结构体不能直接打印
// `derive` 属性会自动自动推导 `Structure` 的 `fmt::Debug` 实现。
// 使这个 `struct` 能使用 `fmt::Debug` 打印。即 {:?} 来打印
// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
#[derive(Debug)]
#[allow(dead_code)]
struct Structure(i32);

// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Structure {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.0)
    }
}

// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

fn main() {
    let x = 5 + 5;
    println!("Hello, world!");

    // 打印变量
    println!("{}", x);

    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 用变量替换字符串有多种写法。比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 让 `Structure` 也可以打印！
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `3` 怎么办？
    println!("Now {:?} will print!", Structure(3));

    // 美化打印
    println!("Now {:#?} will print!", Structure(3));

    // 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。这样，只会打印 3
    println!("Now {} will print!", Structure(3));

    let v = List(vec![6, 7, 8, 9]);
    println!("{}", v);
}
