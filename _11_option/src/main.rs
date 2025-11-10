// 平民（commoner）们见多识广，收到什么礼物都能应对。
// 所有礼物都显式地使用 `match` 来处理。
fn give_commoner(gift: Option<&str>) {
    // 指出每种情况下的做法。
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        // inner 只是一个局部变量名（可以换成其他名），只在该 match 分支的作用域内可用
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

// 养在深闺人未识的公主见到蛇就会 `panic`（恐慌）。
// 这里所有的礼物都使用 `unwrap` 隐式地处理。
fn give_princess(gift: Option<&str>) {
    // `unwrap` 在接收到 `None` 时将返回 `panic`。
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

// 如果 x 是 Option，那么若 x 是 Some ，对x?表达式求值将返回底层值，否则无论函数是否正在执行都将终止且返回 None
fn give_princess_mark(x: Option<&str>) -> Option<&str> {
    // 如果 `x` 是 `None`，这将返回 `None`。
    // 如果 `x` 是 `Some`，内部的 `&str` 将赋值给 `inside`。
    let inside = x?;

    println!("give_princess_mark {}s!!!!!", inside);

    Some(inside)
}

fn main() {
    // Option<T>的枚举类型，用于有 “不存在” 的可能性的情况。它表现为以下两个 “option”（选项）中的一个：
    // Some(T)：找到一个属于 T 类型的元素
    // None：找不到相应元素
    // 这些选项可以通过 match 显式地处理，或使用 unwrap 隐式地处理。隐式处理要么返回 Some 内部的元素，要么就 panic。

    let food  = Some("chicken");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");

    give_princess(bird);

    // snake 和 None 都会触发 panic
    give_princess(bird);
    // give_princess(void);
    // give_princess(snake);

    let x = give_princess_mark(bird);
    println!("{:?}", x)
}
