fn main() {
    // OwnerShip
    // let place1 = "hello";
    // let place2 = "hello".to_string();
    // let other = place1;
    // println!("{:?}", place1);
    // let other = place2;
    // println!("{:?}", place2);

    // Borrow
    let a = [1, 2, 3];
    let b = &a;
    println!("{:p}", b);
    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(4);
    println!("{:?}", d);
    let e = &42;
    assert_eq!(42, *e);
}
