fn main() {
    // OwnerShip
    // let place1 = "hello";
    // let place2 = "hello".to_string();
    // let other = place1;
    // println!("{:?}", place1);
    // let other = place2;
    // println!("{:?}", place2);

    // Borrow
    // let a = [1, 2, 3];
    // let b = &a;
    // println!("{:p}", b);
    // let mut c = vec![1, 2, 3];
    // let d = &mut c;
    // d.push(4);
    // println!("{:?}", d);
    // let e = &42;
    // assert_eq!(42, *e);

    let orig = Box::new(5);
    println!("{}", *orig);
    let stolen = orig;
    println!("{}", *stolen);
    let mut s = String::from("Hello");
    let ss = s.clone();
    s.push_str("string: &str");
    println!("s is {}", s);
    println!("ss is {}", ss);
    let x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);
}
