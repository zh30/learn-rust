fn main() {
    // if
    let y: u32 = 1;
    if y == 1 {
        println!("Y = {}", y);
    }

    // if-else
    if y == 1 {
        println!("Y = {}", y);
    } else {
        println!("Y != 1");
    }

    // if-else if-else
    if y == 1 {
        println!("Y = {}", y);
    } else if y == 0 {
        println!("Y = 0");
    } else {
        println!("Y != 1")
    }

    // let 中使用 if
    let condition: bool = true;
    let x = if condition { 5 } else { 6 };
    println!("X = {}", x);

    // loop
    let mut counter: u32 = 0;
    loop {
        println!("In Loop");
        if counter > 10 {
            break;
        }

        counter += 1;
    }

    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("Result is {}", result);

    // while
    let mut i: u32 = 0;
    while i != 10 {
        i += 1;
    }
    println!("I is {}", i);

    // for
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    // for elem in arr.iter() {
    for elem in &arr {
        println!("Elem is {}", elem);
    }
    for elem in arr.iter().rev() {
        println!("Rev ele is {}", elem);
    }
    'outer: for x in 15..50 {
        for y in 0..16 {
            println!("x is {}, y is {}", x, y);
            if x == y {
                break 'outer;
            }
        }
    }
    println!("Hello, world!");
}
