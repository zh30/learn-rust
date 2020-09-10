fn main() {
	let width: u32 = 30;
	let height: u32 = 50;
	println!("The area is {}", area(width, height));

	let rect1 = Rectangle { width, height };
	let rect2 = Rectangle {
		width: 10,
		height: 40,
	};
	println!("The struct area is {}", struct_area(&rect1));
	println!("The fn area is {}", rect1.area());
	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	let sq = Rectangle::square(width);
	println!("The square is {:#?}", sq);
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	fn can_hold(&self, s: &Rectangle) -> bool {
		self.width * self.height > s.width * s.height
	}
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
		}
	}
}

fn area(width: u32, height: u32) -> u32 {
	width * height
}

fn struct_area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}
