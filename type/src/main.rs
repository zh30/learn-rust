use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    // 布尔类型
    let _is_true: bool = true; // 显示类型声明
    let is_true = false; // 隐式类型声明
    println!("is_true is {}", is_true);

    // char 在 Rust 语言里面，是 32 位的字符类型
    let a: char = 'a';
    println!("a is {}", a);

    let b: char = '你';
    println!("b is {}", b);

    // i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
    // 整数类型
    let _c: i8 = -111; // 类型声明
    let _c = -37i8; // 类型后缀声明
    let _c = 25; // 默认是 i32 类型
    let _c: u32 = 0b10001; // 二进制
    let _c: u32 = 0o21; // 八进制
    let _c: u32 = 0x11; // 十六进制
    let c = 38_000; // 数字可读性
    println!("c is {}", c);
    // 浮点数类型
    let _d: f32 = 0.0009; // 类型声明
    let _d = 0.0009f32; // 类型后缀声明
    let _d = 0.0009; // 默认f64类型
    let d = 1_000.0_009; // 数字可读性分隔符_
    println!("d is {}", d);

    // 自适应类型 isize, usize
    println!("usize max is {}", usize::max_value());
    println!("isize max is {}", isize::max_value());

    // 范围类型
    print!("(1..5): "); // 左闭右开区间
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();
    print!("(1..=5): "); // 全闭区间
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    let sum: i32 = (1..=5).sum();
    println!("(1..=5).sum(): {}", sum);

    // 数组
    // [Type; size] type 代表元素类型，size 代表长度即元素的个数
    let _arr: [u32; 5] = [1, 2, 3, 4, 5];
    let _arr = [1, 2, 3, 4, 5];
    let arr = [1; 5]; // 等价于 let arr = [1, 1, 1, 1, 1];
    println!("arr[0] is {}", arr[0]); // Vec 动态数组，允许增加和缩短长度的容器类型。

    show(arr);

    // 元组
    let tup: (i32, f32, char) = (3, 4.33, '我'); // 由一个或多个类型的元素组合成的复合类型，使用()把所有元素放在一起。元素之间用,分割。
    println!("tup.0 is {}", tup.0); // 使用“元组名.索引”来访问元组中的相应索引位置的元素，从0开始计数。

    let (x, y, z) = tup; // 解构
    println!("x y z is {:1} {:2} {:3}", x, y, z);

    let zh: f32 = 0.1;
    let he: f32 = 0.2;
    println!("zh + he = {}", zh + he);

    // 结构体类型是一个自定义数据类型，通过struct关键字加自定义名字命名
    struct Student {
        name: &'static str,
        score: u32,
    }
    let mut student = Student {
        name: "zhanghe",
        score: 29,
    };
    student.name = "liunan";
    println!("student name is: {}", student.name);

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Black is {}", black.0);

    // 枚举类型是一个自定义数据类型，通过enum关键字加自定义命名来定义。
    #[derive(Debug)]
    enum ColorNoParam {
        Red(String),
        // Yellow(String),
        // Blue(String),
    }
    let color_no_param = ColorNoParam::Red(String::from("red"));
    println!("color_no_param red is {:?}", color_no_param);

    // Vec 一种动态可变长度数组
    let mut _v: Vec<i32> = Vec::new();
    let mut _v: Vec<i32> = Vec::with_capacity(10);
    let mut _v = vec![1, 2, 3];
    let mut v = vec![1; 10];
    v.push(2); // 尾部增加一个2
    v[1] = 5; // 索引为1的元素修改为5
    let popNum = v.pop(); // 删除并返回动态数组的最后一个元素，如果为空则返回None。
    let removeNum = v.remove(1);
    println!("vec remove is {:?}", removeNum);
    println!("vec pop is {:?}", popNum);
    println!("vec is {:?}", v);
    println!("vec 3 is {:?}", v[3]); // 访问动态数组里面的元素
    println!("vec 5 is {:?}", v.get(5)); // 同上
    vecShow(v);

    // 双端队列
    let mut vd: VecDeque<u32> = VecDeque::new();
    let mut _vd: VecDeque<u32> = VecDeque::with_capacity(10);
    vd.push_back(1);
    vd.push_back(2);
    vd.push_front(1);
    vd.push_front(2);
    vd[1] = 5; // 索引为1的元素修改为5
    let popBack = vd.pop_back(); // 删除并返回队列尾部的元素。
    let popFront = vd.pop_front(); // 删除并返回队列头部的元素。
    let removeNum = vd.remove(1); // 删除并返回队列指定索引的元素，如果索引越界，则返回None。
    println!("vd remove is {:?}", removeNum);
    println!("vd popBack is {:?}", popBack);
    println!("vd popFront is {:?}", popFront);
    println!("vd: {:?}", vd);
    println!("vd 0 is {:?}", vd[0]); // 访问队列里面的元素，如果索引越界，则会导致程序错误。
    println!("vd 0 is {:?}", vd.get(0)); // 访问队列里面的元素，如果索引越界，则返回None。

    // 哈希表
    let mut map: HashMap<&str, i32> = HashMap::new();
    let mut _map: HashMap<&str, i32> = HashMap::with_capacity(10);
    let zhanghe = map.insert("zhanghe", 29); // 插入或者更新操作。如果不存在，执行插入并返回None。如果键已存在，执行更新操作，将对应键的值更新并返回旧值。
    map.insert("liunan", 29);
    println!("zhanghe is {:?}", zhanghe);
    map.entry("zhanghe").or_insert(29); // 检查键是否有对应值，没有对应值就插入键-值对，有对应值则不执行任何操作。
    map.entry("lisan").or_insert(18);
    println!("map is {:?}", map);
    for (_, val) in map.iter_mut() {
        *val += 2;
    }
    let mapResult = map.remove("lisan"); // 删除并返回指定的键值对，如果不存在就返回 None。
    println!("mapResult is {:?}", mapResult);
    println!("zhanghe is {:?}", map["zhanghe"]); // 访问 map 中的指定键，如果不存在，会导致程序错误。
    println!("zhanghe is {:?}", map.get("zhanghe")); // 以键为参数访问指定的键值对，如果不存在，将会返回None。
    println!("hash is {:?}", map);

    // 字符串
    // Rust 的常用字符串有两种，一种是固定长度的字符串字面量str，一种是可变长度的字符串对象String。
    let _s1 = "Hello, rust!"; // &str的创建（使用双引号创建字符串字面量）
    let str = String::from("Hello, rust!");
    let _s1 = str.as_str(); // 使用 as_str 方法将字符串对象转化为字符串字面量。

    let mut s2 = String::new(); // 字符串对象 String 是由 Rust 标准库提供的、拥有所有权的 UTF-8 编码的字符串类型，创建后可以为其追加内容或更改内容
    let mut s = String::from("Hello, Rust!"); // 使用 String::from 函数根据指定的字符串字面量创建字符串对象
    let _str = "Hello, Rust!";
    let _s = str.to_string(); // 使用 to_string 方法将字符串字面值转化为字符串对象。
                              // s.push(' ');
                              // s.push_str("Wow!");
    s.insert(5, ','); // 在字符串中插入字符，如果索引非法，则导致程序错误
    s.insert_str(7, "Rust"); // 在字符串中插入字符串字面量，如果索引非法，则导致程序错误

    let mut sss = str + &s; // 连接与追加的区别在于，连接会返回新的字符串，而不是在原字符串上的追加。
    let strs = format!("{}-{}", sss, s); // 较为复杂或带有格式的字符串连接，我们可以使用格式化宏format!，它对于String类型和&str类型的字符串都适用
    println!("strs is {}", strs);
    println!("sss is {}", sss);
    let mut s1 = s.replace(" ", "hahaha"); // 将字符串中的指定的子串替换为另一个字符串。
    let mut s2 = s.replacen(" ", "lilili", 1);
    println!("{}", s1);
    println!("{}", s2);
    println!("{:?}", s1.pop());
    println!("{}", s2.remove(3));
    s2.truncate(4);
    println!("{}", s2);
    println!("s len: {}", s.len());
    println!("string is {:?}", s);
    for b in s.bytes() {
        println!("{} | ", b);
    }
    for c in s.chars() {
        println!("{} | ", c);
    }
}

fn show(arr: [u32; 5]) {
    println!("---------------------------");
    for i in &arr {
        println!("{}", i);
    }
    println!("---------------------------");
}

fn vecShow(arr: Vec<i32>) {
    println!("---------------------------");
    for i in &arr {
        println!("{}", i);
    }
    println!("---------------------------");
}

// fn hashIter(hash: HashMap<&str, i32>) {
//     println!("---------------------------");
//     for (_, val) in hash.iter_mut() {
//         *val += 2;
//     }
//     println!("hash is {:?}", hash);
//     println!("---------------------------");
// }
