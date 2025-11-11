// 变量要负责释放它们拥有的资源，所以资源只能拥有一个所有者。这也防止了资源的重复释放。注意并非所有变量都拥有资源（例如引用）。
// 在进行赋值（let x = y）或通过值来传递函数参数（foo(x)）的时候，资源的所有权（ownership）会发生转移。按照 Rust 的说法，这被称为资源的移动（move）。
// 重要：移动资源之后，原来的所有者不能再被使用

// 此函数取得堆分配的内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
    // `c` 被销毁且内存得到释放
}

fn main() {
    // 栈分配的整型
    let x = 5u32;

    // 将 `x` *复制*到 `y`——不存在资源移动
    let y = x;

    // 两个值各自都可以使用
    println!("x is {}, and y is {}", x, y);

    // `a` 是一个指向堆分配的整数的指针
    let a = Box::new(5i32);

    println!("a 拥有 {}", a);

    // *移动* `a` 到 `b`
    // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
    // 同一个堆分配的数据，但是现在是 `b` 拥有它。
    let b = a;

    // 重要：报错
    // 现在 `a` 不能访问数据，因为它不再拥有那部分堆上的内存。
    // println!("a 拥有: {}", a);

    // 此函数从 `b` 中取得堆分配的内存的所有权
    // 传统语言这里实际上是个*复制值*的过程，这里更像是把他拿走了。
    destroy_box(b);

    // 重要：报错
    // 现在 `b` 不能访问数据，因为它不再拥有那部分堆上的内存。
    // 现在堆内存已经被释放了。
    // println!("b contains: {}", b);

    // 当所有权转移时，数据的可变性可能发生改变。
    // immutable_box 是不可变的
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // 可变性错误
    //*immutable_box = 4;

    // *移动* box，改变所有权，同时改变了 *可见性*
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // 修改 box 的内容
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
