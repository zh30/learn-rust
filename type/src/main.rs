fn main() {
    let is_true: bool = true;
    println!("is_true is {}", is_true);

    // char 在 Rust 语言里面，是 32 位的
    let a: char = 'a';
    println!("a is {}", a);

    let b: char = '你';
    println!("b is {}", b);

    // i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
    let c: i8 = -111;
    println!("c is {}", c);

    let d: f32 = 0.0009;
    println!("d is {}", d);

    // 自适应类型 isize, usize
    println!("usize max is {}", usize::max_value());
    println!("isize max is {}", isize::max_value());

    // 数组
    // [Type: size] size 也是数组的一部分
    let arr: [u32; 5] = [1,2,3,4,5];
    println!("arr[0] is {}", arr[0]);

    show(arr);

    // 元组
    let tup: (i32, f32, char) = (3, 4.33, '我');
    println!("tup.0 is {}", tup.0);

    let (x, y, z) = tup;
    println!("x y z is {:1} {:2} {:3}", x, y, z);

    let zh: f32 = 0.1;
    let he: f32 = 0.2;
    println!("zh + he = {}", zh + he);
}

fn show(arr: [u32; 5]) {
    println!("---------------------------");
    for i in &arr {
        println!("{}", i);
    }
    println!("---------------------------");
}
