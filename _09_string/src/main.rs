fn main() {
    // Rust 中有两种字符串类型：String 和 &str。
    //
    // String 被存储为由字节组成的 vector（Vec<u8>），但保证了它一定是一个有效的 UTF-8 序列。String 是堆分配的，可增长的，且不是零结尾的（null terminated）。
    //
    // &str 是一个总是指向有效 UTF-8 序列的切片（&[u8]），并可用来查看 String 的内容，就如同 &[T] 是 Vec<T> 的全部或部分引用。

    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // 逆序迭代单词，这里并没有分配新的字符串
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // 复制字符到一个 vector，排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    println!("{:?}", chars);

    // 创建一个空的且可增长的 `String`
    let mut string = String::new();
    for c in chars {
        // 在字符串的尾部插入一个字符
        string.push(c);
        // 在字符串尾部插入一个字符串
        string.push_str(", ");
    }
    println!("{}", string);


    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    // 分配新内存并存储修改过的字符串
    let bob: String = alice.replace("dog", "cat");
    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    // 长字符串
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    // 有时会有太多需要转义的字符，或者是直接原样写出会更便利。这时可以使用原始字符串（raw string）。
    let escapes_str = "Escapes don't work here: \x3F \u{211D}";
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", escapes_str);
    println!("{}", raw_str);
}
