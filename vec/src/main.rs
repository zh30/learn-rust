fn main() {
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);
    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v1[1], 2);
    let mut v2 = vec![0; 10];
    v2[1] = 1;
    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);
    assert_eq!(v2, [1, 2, 3]);
}
