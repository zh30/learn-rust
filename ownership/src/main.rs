fn main() {
    // OwnerShip
    // let place1 = "hello";
    // let place2 = "hello".to_string();
    // let other = place1;
    // println!("{:?}", place1);
    // let other = place2;
    // println!("{:?}", place2);

    // Borrow
    let a = [1,2,3];
    let b = &a;
    println!("{:p}", b);
}
