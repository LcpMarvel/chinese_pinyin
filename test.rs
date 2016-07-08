use std::iter::FromIterator;

fn main() {

	let s = "中国";

	let chars: Vec<char> = s.chars().collect();


	let ss: String = chars.into_iter().collect();
	println!("{:?}", ss);


}
