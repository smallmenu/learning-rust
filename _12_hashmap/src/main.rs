use std::collections::HashMap;

fn main() {
    // 和 vector 类似，HashMap 也是可增长的，但 HashMap 在占据了多余空间时还可以缩小自己。
    // 可以使用 HashMap::with_capacity(unit) 创建具有一定初始容量的 HashMap，也可以使用 HashMap::new() 来获得一个带有默认初始容量的 HashMap（这是推荐方式）。
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    println!("Contact is {:?}", contacts);
}
