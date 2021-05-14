fn main() {
    // 字面量
    // 字面量是指由文字、数字或符号构成的值，它只能作为等号右边的值出现。比如整数1、浮点数1.2、字符'a'、字符串"abc"、布尔值true和单元值()都是字面量。
    // 一个函数无返回值，实际上是以单元值作为函数的返回值了。
    let sum = 3 + 5; // + 是运算符，3 和 5 是操作数，8 是运算符操作的结果。
    println!("3+5={}", sum);
    // 算术运算符：加减乘除，
    // if
    let y: u32 = 1;
    // 关系运算符：比较两个值之间的关系，并返回一个布尔类型的值。
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

    // 逻辑运算符：与或非
    if y > 0 && y < 3 {
        println!("Y is {}", y);
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
