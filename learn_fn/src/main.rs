fn main() {
    hi();
    let a: i32 = -1;
    let b: i32 = 2;
    hi1(a, b);
    let c: i32 = hi2(a, b);
    println!("c is {}", c);
    hi3(a, b);
    println!("Hello, world!");
}

fn hi() {
    println!("this is function")
}

fn hi1(a: i32, b: i32) {
    println!("a is {:1}, b is {:2}", a, b);
}

fn hi2(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}

fn hi3(a: i32, b: i32) -> i32 {
    a + b
}
