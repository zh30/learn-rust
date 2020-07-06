use std::cmp::{max, min};
// use std::{cmp::min, cmp::max};
// use std::cmp::*;

struct Vec2 {
    x: f64,
    y: f64,
}

struct Number {
    odd: bool,
    value: i32,
}

fn main() {
    // 定义常量
    const MAX: u32 = 10000;
    println!("MAX is {}", MAX);
    let x: i32 = 42;
    let y: i32 = 32;
    let x = y + x;
    println!("{:?}", x);
    let pair = ("zhanghe", 16);
    println!("{parione}, {paritwe}", parione = pair.0, paritwe = pair.1);
    let zh: (char, i32) = ('l', 16);
    println!("{zhone}, {zhtwe}", zhone = zh.0, zhtwe = zh.1);
    let (some_char, some_int) = ('a', 16);
    println!(
        "{somechar}, {someint}",
        somechar = some_char,
        someint = some_int
    );

    greet();
    fair_dice_roll();
    {
        let x: i32 = 33;
        println!("{inner_x}", inner_x = x);
    }
    println!("{outer_x}", outer_x = x);

    let z: i32 = { x + y };
    println!("{:?}", z);

    let least = min(3, 8);
    let m = max(3, 8);
    println!("{lnum} {mnum}", lnum = least, mnum = m);

    let l = "zhang".len();
    // let l = str::len("zhanghe");
    println!("{:?}", l);

    // let v = Vec::new();
    // let v = std::vec::Vec::new();
    // println!("{:?}", v);

    let v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { x: 3.6, y: 4.0 };
    let v3 = Vec2 { x: 14.0, ..v2 };
    let v4 = Vec2 { ..v3 };
    let Vec2 { x: _, y: _ } = v1;
    let Vec2 { x: _, .. } = v2;

    let one = Number {
        odd: true,
        value: 1,
    };
    let two = Number {
        odd: false,
        value: 2,
    };
    print_number(one);
    print_number(two);
}

fn greet() {
    println!("Hi! i'm zhanghe");
}

fn fair_dice_roll() -> i32 {
    let feeling_lucky: bool = true;
    if feeling_lucky {
        6
    } else {
        4
    }
    // match feeling_lucky {
    //     true => 6,
    //     false => 4,
    // }
}

fn print_number(n: Number) {
    match n {
        Number { odd: true, value } => println!("Odd number: {}", value),
        Number { odd: false, value } => println!("Even number: {}", value),
    };
    // match n {
    //     Number { value: 1, .. } => println!("One"),
    //     Number { value: 2, .. } => println!("Two"),
    //     Number { value, .. } => println!("{}", value),
    // };
    // match n.value {
    //     1 => println!("One"),
    //     2 => println!("Two"),
    //     _ => println!("{}", n.value),
    // };
    // if let Number { odd: true, value } = n {
    //     println!("Odd number: {}", value);
    // } else if let Number { odd: false, value } = n {
    //     println!("Even number: {}", value);
    // };
}
