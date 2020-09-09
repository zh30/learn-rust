fn main() {
    let mut s = String::from("hello world");
    let t = "hi zhanghe";

    let world = first_word(&s); // let world = first_word(&s[..])
    println!("first word is \"{}\"", world);

    let he = fw(t);
    // let he = fw(&t[..]);
    println!("he first is {}", he);

    let hello = &s[..5]; // &s[0..5]
    let word = &s[6..]; // &s[6..11]
    let all = &s[..]; // &s[0..11]
    println!(
        "hello is \"{}\", word is \"{}\", all is \"{}\"",
        hello, word, all
    );

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    analyze_alice(&a[1..]); // 数组的切片

    s.clear(); // 这里的 clear 方法会清空当前字符串，使之变为 ""
}

fn first_word(s: &String) -> &str {
    // fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn fw(s: &str) -> &str {
    // fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn analyze_alice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
