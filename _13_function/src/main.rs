// 函数的参数需要标注类型，就和变量一样，如果函数返回一个值，返回类型必须在箭头 -> 之后指定。
// 一个返回布尔值的函数
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 边界情况，使用 return 提前返回
    if rhs == 0 {
        return false;
    }

    // 这是一个表达式，可以不用 `return` 关键字
    lhs % rhs == 0
}

// 没有返回值的函数。实际上会返回一个单元类型 `()`，这时候可以省略
fn fizzbuzz(n: u32) {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn main() {
    let a = is_divisible_by(11, 5);
    println!("is_divisible: {}", a);

    // 我们可以在这里使用函数，后面再定义它
    fizzbuzz(100);
}

