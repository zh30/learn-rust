fn main() {
    println!("Hello, world!");
    let mut x: i32 = 6;
    print!("x is {}", x);
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        println!(" -> {x}");
    }
    println!("");
}
