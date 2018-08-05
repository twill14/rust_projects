enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn main() {
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
    	print!("three");
    }


    let mut Count = 0;

    match coin {
    	Coin::Quarter(state) => println!("State Quarter from {:?}", state),
    	_ => += 1,
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
    	println!("State Quarter from {:?}", state);
    }
    else {
    	count += 1;
    }
}
