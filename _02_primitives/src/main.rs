// 有符号整数（signed integers）：i8、i16、i32、i64、i128 和 isize（指针宽度）
// 无符号整数（unsigned integers）： u8、u16、u32、u64、u128 和 usize（指针宽度）
// 浮点数（floating point）： f32、f64
// char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
// bool（布尔型）：只能是 true 或 false
// 单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

// 常量, 不可改变的值
const THRESHOLD: i32 = 10;

fn main() {
    // 变量可以给出类型说明。
    let logical: bool = true;

    // 常规说明
    let a_float: f64 = 1.0;

    // 后缀说明
    let an_integer   = 5i32;

    // 否则会按默认方式决定类型。编译器会对整数使用 i32，对浮点数使用 f64。
    let default_float   = 3.0;

    let default_integer = 7;

    // 可变的（mutable）变量，其值可以改变。Mutable `i32`
    let mut mutable = 12;

    println!("mutable: {}", mutable);

    // 改变值
    mutable = 21;

    println!("mutable: {}", mutable);

    // 报错！变量的类型并不能改变。
    // mutable = true;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;

    println!("{}, {}, {}, {}, {}, {}", logical, a_float, an_integer, default_float, default_integer, mutable);

    // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减， 报错 overflow
    // println!("1 - 2 = {}", 1u32 - 2);

    // 使用下划线改善数字的可读性！
    println!("One million is written as {}", 1_000_000u32);

    // 包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    // 元组可以打印
    println!("long_tuple: {:?}", long_tuple);

    // 通过元组的下标来访问具体的值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.10);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组无法打印
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    // 定长数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 50] = [3; 50];

    // 下标从 0 开始
    println!("second element of the array: {}", xs[1]);
    println!("second element of the array: {:?}", ys);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的，4 * 5 = 20 bytes
    println!("array occupies {} bytes", size_of_val(&xs));

    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);


    println!("This is {}", THRESHOLD);
}
