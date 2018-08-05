enum Coin{
	Penny,
	Nickel,
	Dime,
	Quarter,
}

fn value_for_cents(coin: Coin) -> u32 {
	match coin {
		Coin::Penny => {
			println!("This is a Penny");
			1
		},

		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter => 25,
	}
}

fn main() {
    value_for_cents(Coin::Penny);
}
