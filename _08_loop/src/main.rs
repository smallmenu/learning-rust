#![allow(unreachable_code)]
#![allow(unused_labels)]
#![allow(unused_variables)]

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Rust 提供了 loop 关键字来表示一个无限循环。
    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }

    // 处理嵌套循环的时候可以 break 或 continue 外层循环，必须用一些 'label（标签）来注明
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    // loop 有个用途是尝试一个操作直到成功为止，是个表达式，通过 break 返回。
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    //while 关键字可以用作当型循环
    let mut n = 1;

    // 当 `n` 小于 101 时循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加 1
        n += 1;
    }

    // for in 结构可以遍历一个 Iterator（迭代器）
    // a..b = [a, b)
    // a..=b = [a, b]
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }


    let names = vec!["Bob", "Frank", "Ferris"];

    // iter() - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用。
    // into_iter() - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了
    // iter_mut() - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // 没有变化
    println!("names: {:?}", names);

    // 这是使用 iter_mut() 的方法，可以修改值
    let mut mutNames = vec!["Bob", "Frank", "Ferris"];

    for name in mutNames.iter_mut() {
        // *name 解引用并直接修改其值
        *name = "abc";
    }

    println!("mutNames: {:?}", mutNames);
}
