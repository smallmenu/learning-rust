// 一个具体类型 `A`。
#[derive(Debug)]
struct A;

// 因此，`Single` 也是个具体类型，`A` 取上面的定义。
struct Single(A);
// ^ 这里是 `Single` 对类型 `A` 的第一次使用。

// 此处 `<T>` 在第一次使用 `T` 前出现，所以 `SingleGen` 是一个泛型类型。
// 因为 `T` 是泛型的，所以它可以是任何类型，包括在上面定义的具体类型 `A`。
#[derive(Debug)]
struct SingleGen<T>(T);

fn main() {
    // `Single` 是具体类型，并且显式地使用类型 `A`。
    let _s = Single(A);

    // 这里的 `SingleGen` 的类型参数是显式指定的。
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` 的类型参数也可以隐式地指定。
    let _t = SingleGen(A); // 使用在上面定义的 `A`。
    let _i32 = SingleGen(6); // 使用 `i32` 类型。
    let _char = SingleGen('a'); // 使用 `char`。
    let _string = SingleGen("asdf"); // 使用 `char`。

    println!("The value of age is {:?}", _t);
    println!("The value of age is {:?}", _i32);
    println!("The value of age is {:?}", _char);
    println!("The value of age is {:?}", _string);
}
