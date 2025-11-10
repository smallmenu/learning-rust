// Rust 使用 trait 解决类型之间的转换问题。最一般的转换会用到 From 和 Into 两个 trait


use std::convert::From;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}


// 把任何类型转换成 String，不需要实现那个类型的 ToString trait。应该实现fmt::Display trait
// 它会自动提供 ToString，并且还可以用来打印类型
struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    // From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，
    // 它提供了一种类型转换的简单机制。在标准库中有无数 From 的实现，规定原生类型及其他常见类型的转换功能。

    let num = Number::from(30);
    println!("My number is {:?}", num);

    // 如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。
    let i = 5;

    // Into 要求指明要转换到的类型
    let num: Number = i.into();
    println!("My number is {:?}", num);

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // 字符串转成数字，标准手段是用 parse 函数。默认会转换为 i32
    // 提供要转换到的类型，这可以通过不使用类型推断，
    let parsed:i64 = "5".parse().unwrap();
    // 或者用 “涡轮鱼” <> 语法
    let turbo_parsed = "10".parse::<i64>().unwrap();

    let sum:i64 = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}
