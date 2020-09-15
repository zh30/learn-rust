#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}

mod front_of_house {
	mod hosting {
		fn add_to_waitlist() {
			println!("add_to_waitlist");
		}
		fn seat_at_table() {
			println!("seat_at_table");
		}
	}
	mod serving {
		fn take_order() {
			println!("take_order");
		}
		fn searve_order() {
			println!("searve_order");
		}
		fn take_payment() {
			println!("take_payment");
		}
	}
}
