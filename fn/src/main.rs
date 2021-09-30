#[derive(Debug)]
pub struct Student {
    name: &'static str,
    score: i32,
}

impl Student {
    pub fn new(name: &'static str, score: i32) -> Self {
        Student {name, score}
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    pub fn set_score(&mut self, score: i32) {
        self.score = score;
    }
}

fn main() {
    hi();
    let a: i32 = -1;
    let b: i32 = 2;
    hi1(a, b);
    let c: i32 = hi2(a, b);
    println!("c is {}", c);
    let r = hi3(a, b);
    println!{"R = {}", r};
    println!("Hello, world!");
    let mut student: Student = Student::new("zhanghe", 29);
    println!("name: {}, score: {}", student.get_name(), student.get_score());
    student.set_score(30);
    println!("student is {:?}", student);
}

fn hi() {
    println!("this is function")
}

fn hi1(a: i32, b: i32) {
    println!("a is {:1}, b is {:2}", a, b);
}

fn hi2(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}

fn hi3(a: i32, b: i32) -> i32 {
    a + b
}
