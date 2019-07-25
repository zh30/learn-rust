// 引用标准库std的io
use std::io;
// 引入标准库std的cmp
use std::cmp::Ordering;
// 引入rand
use rand::Rng;
// main() 程序的入口点
fn main() {
	// 打印一行文字
	println!("猜数字！");
	// rand::thread_rng 特定随机数生成器。
	let secret_number = rand::thread_rng().gen_range(1, 101);
	// loop 创建了无限循环，break结束当前loop。
	loop {
		// 打印一行文字
		println!("请输入你的猜测数字：");
		// 声明一个变量（使用 let ）。mut 代表是可变的，不加则相反。= 号右边为 String 类型的空字符串。
		let mut guess = String::new();
		// 从模块 io 中调用 stdin 函数，调用标准输入句柄获取用户的输入文字，把获取的输入文本赋值给变量 guess。
		io::stdin().read_line(&mut guess).expect("输入失败！");
		// 使用trim函数去除回车空格，使用parse把字符串转为数字。{}处理无效输入。
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		// 打印用户输入的文本。
		println!("你猜的是: {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("太小了！"),
			Ordering::Greater => println!("太大了！"),
			Ordering::Equal => {
				println!("你赢了");
				// 打印生成的随机数。
				println!("要猜的数： {}", secret_number);
				break;
			}
		}
	}
}
