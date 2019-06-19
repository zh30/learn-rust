// 引用标准库std的io
use std::io;
// main() 程序的入口点
fn main() {
	// 打印一行文字
	println!("Guess the number:!");
	// 打印一行文字
	println!("Please input your guess.");
	// 声明一个变量（使用 let ）。mut 代表是可变的，不加则相反。= 号右边为 String 类型的空字符串。
	let mut guess = String::new();
	// 从模块 io 中调用 stdin 函数，调用标准输入句柄获取用户的输入文字，把获取的输入文本赋值给变量 guess。
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	// 打印用户输入的文本。
	println!("Your guessed: {}", guess);
}
