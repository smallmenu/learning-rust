// Rust 的变量不只是在栈中保存数据：它们也占有资源，比如 Box<T> 占有堆（heap）中的内存。
// Rust 强制实行 RAII（Resource Acquisition Is Initialization，资源获取即初始化）。
// 任何对象在离开作用域时，它的析构函数（destructor）就被调用，然后它占有的资源就被释放。

fn create_box() {
    // 在堆上分配一个整型数据
    let _box1 = Box::new(3i32);

    // `_box1` 在这里被销毁，内存得到释放
}

// Rust 中的析构函数概念是通过 Drop trait 提供的。当资源离开作用域，就调用析构函数。
struct ToDrop;

// 你无需为每种类型都实现 Drop trait，只要为那些需要自己的析构函数逻辑的类型实现就可以了。
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    // 在堆上分配一个整型数据
    let _box2 = Box::new(5i32);

    // 嵌套作用域：
    {
        // 在堆上分配一个整型数据
        let _box3 = Box::new(4i32);

        // `_box3` 在这里被销毁，内存得到释放
    }

    // 创建一大堆 box（只是因为好玩）。
    // 完全不需要手动释放内存！
    for _ in 0u32..1_000 {
        create_box();
    }

    let x = ToDrop;


    // `_box2` 在这里被销毁，内存得到释放
    //
}
