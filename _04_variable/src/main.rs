fn main() {
    // Rust 通过静态类型确保类型安全。变量绑定可以在声明时说明类型，编译器能够从上下文推导出变量的类型
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 变量绑定默认是不可变的（immutable）, 编译期报错
    // an_integer = 32

    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
    // let unused_variable = 3u32;

    // 这样就不会有警告了。
    let _unused_variable = 3u32;


    let mut mutable_binding = 1;

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 此绑定生存于 main 函数中
    let long_lived_binding = 1;

    // 这是一个代码块，比 main 函数拥有更小的作用域
    {
        // 此绑定只存在于本代码块
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // 此绑定*遮蔽*了外面的绑定，内部*遮蔽*并不影响外部，*遮蔽*可以理解为使用相同的名字重新声明一个绑定。
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // 代码块结束

    // 报错！`short_lived_binding` 在此作用域上不存在
    // println!("outer short: {}", short_lived_binding);

    // 内部*遮蔽*并不影响外部
    println!("outer long: {}", long_lived_binding);

    // 此绑定同样*遮蔽*了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);

    // _mutable_integer 是可变的
    let mut _mutable_integer = 7i32;

    println!("before _mutable_integer: {}", _mutable_integer);

    {
        // 在内部，被不可变的 `_mutable_integer` 遮蔽，即被相同的名称不变地绑定时，它会冻结
        let _mutable_integer = _mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        // _mutable_integer = 50;
        // 改正 ^ 注释掉上面这行

        // `_mutable_integer` 离开作用域
    }

    // 正常运行！ `_mutable_integer` 在这个作用域没有冻结
    _mutable_integer = 3;

    println!("after _mutable_integer: {}", _mutable_integer);
}
