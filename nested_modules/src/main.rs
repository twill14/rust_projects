pub mod a {
	pub mod series {
		pub mod of {
			pub fn nested_modules() {}
		}
	}
}

enum TrafficLight {
	Red,
	Yellow,
	Green,
}

use a::series::of;

use TrafficLight::{Red, Yellow};

// Or you can use

use TrafficLight::*

// Which makes a short cut to reference all members of an enum

fn main() {
    of::nested_modules;

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}


