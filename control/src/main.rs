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

    // if let 处理只匹配一个模式的值而忽略其他模式的情况。
    let zle = Some(8);
    if let Some(8) = zle {
        println!("IF LET is run!");
    }

    // while let
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    while let Some(value) = vec.pop() {
        println!("while let is {}", value);
    }

    // let 中使用 if
    let condition: bool = true;
    let x = if condition { 5 } else { 6 };
    println!("X = {}", x);

    // loop，重复执行，永远不会结束的循环。
    let mut counter: u32 = 0;
    loop {
        println!("In Loop {}", counter);
        if counter > 10 {
            break; // 循环控制语句，用于退出循环并将返回值返回。
        }

        counter += 1;
    }

    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2; // 退出循环并返回值。
        }
    };
    println!("Result is {}", result);

    // while
    let mut i: u32 = 0;
    while i <= 10 {
        i += 1;
        println!("While is {}", i);
    }

    // for ... in ...的语法，是一种重复执行指定次数的循环。因其安全性和简洁性常用于对范围类型或集合类型的所有元素执行指定的操作。
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    // for elem in arr.iter() {
    for elem in &arr {
        println!("Elem is {}", elem);
    }
    for elem in arr.iter().rev() {
        println!("RevEle is {}", elem);
    }
    'outer: for x in 15..50 {
        for y in 0..17 {
            println!("x is {}, y is {}", x, y);
            if x == 15 || y == 15 {
                continue;
            };
            if x == y {
                break 'outer; // break语句直接退出循环，不再执行循环体内的任何代码。而continue语句仅是跳出当前轮循环，不再执行循环体内continue语句之后的代码，但它会再次进行条件判断，决定是否重复执行循环。
            }
        }
    }
    println!("Hello, world!");
}
