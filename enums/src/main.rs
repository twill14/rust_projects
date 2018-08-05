// Enum type 1 Versus Struct

enum IpAddrKind {
	V4,
	V6,
}

struct IpAddr {
	kind: IpAddrKind,
	address: String,
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr{
    	kind: IpAddrKind::V4,
    	address: String::from("127.0.0.1");
    }

    let loopback = IpAddr{
    	kind: IpAddrKind::V6,
    	address: String::from("::1"),
    }
}

fn route(ip_type: IpAddrKind){ }

----------------------------------------------------------------------------------

// Enum type 2

enum IpAddr {
	V4(String),
	V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

----------------------------------------------------------------------------------

// Enum type 3

enum Message {
	Quit,
	Move (x: i32, y: i32),
	Write (String),
	ChangeColor(i32, i32, i32),
}

enum Job {
	Mage(),
	Warrior(),
	Rogue(),
	Ranger(),
	Ninja(),
	Samurai(),
}

struct Warrior {
	Class: Job,
	LVL: u32,
	HP: u32,
	MP: u32,
	SPD: u32,
	DEF: u32,
	RES: u32,
	DEX: u32,
}

enum Warrior {
	Class(Job),
	LVL(u32),
	HP(u32),
	MP(u32),
	SPD(u32),
	DEF(u32),
	RES(u32),
	DEX(u32),
}

// Enums and Structs are both good for keeping track of data.
// Enums are better at storing elements which can have unrelated functions. It's also better for storing elements which have very different data types and functions.
// Structs seem to only be best used when each of the elements are an important part of the Struct object as a whole.

// After review my final conclusion is: Enums can accomplish everything structs can and a little bit more. Therefore, I'll just only use enums in case Structs get deprecated down the line.

-----------------------------------------------------