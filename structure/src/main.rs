fn main() {
    let mut user1 = User {
        email: String::from("zhanghecool@outlook.com"),
        name: String::from("zhanghe"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("zhanghelook@outlook.com");
    user1.active = false;
    user1.sign_in_count = 2;
    println!("user1 name is : {}", user1.name);
    let zhanghe: User = build_user(user1.email, user1.name);
    println!("zhanghe name is : {}", zhanghe.name);

    let black = Color(0, 0,0);
    let origin = Point(0,0,0);
    // println!("{}{}", black, origin);
}

struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: false,
        sign_in_count: 1,
    }
}
