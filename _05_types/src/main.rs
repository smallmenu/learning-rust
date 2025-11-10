// type 语句给已有的类型取个新的名字。类型的名字必须遵循驼峰命名法（像是 CamelCase 这样），否则编译器将给出警告。
// 原生类型命名法可以认为是个例外，比如： usize、f32，等等。

#[allow(dead_code)]
type NanoSecond = u64;

#[allow(dead_code)]
type Inch = u64;

fn main() {
    // Rust 不提供隐式类型转换，可以使用 as 关键字进行显式类型转换
    let decimal = 256.4321f32;

    println!("{}", decimal);

    // 错误！不提供隐式转换
    // let integer: u8 = decimal;

    // 可以显式转换
    let integer = decimal as u8;

    // 可以看到最大只转换为 255
    println!("{}", integer);

    // 转换为 char
    let character = integer as char;

    // 打印出：ÿ
    println!("{}", character);

    // Rust 的类型推断引擎是很聪明的，它不只是在初始化时看看右值（r-value）的类型而已，它还会考察变量之后会怎样使用
    // 借此推断类型。这是一个类型推导的进阶例子：
    // 因为有类型说明，编译器知道 `elem` 的类型是 u8。
    let elem = 5u8;

    // 创建一个空向量（vector，即不定长的，可以增长的数组）。
    let mut vec = Vec::new();
    // 现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<_>`）

    // 在向量中插入 `elem`。
    vec.push(elem);

    // 啊哈！现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）。
    // 试一试 ^ 注释掉 `vec.push(elem)` 这一行。
    println!("{:?}", vec);
    println!("{:?}", size_of_val(&vec));
}
