use std::io;
fn main() {
	//let mut input = String::new();
	//io::stdin().read_line(&mut input).expect("Faild input");
	let mut line_number=0;
	loop {
		print!("{}: ", line_number);
		let mut input = String::new();
		io::stdin().read(&mut input).expect("Faild");
		line_number = line_number+1;
	}
}
