use std::iter::FromIterator;

fn main() {

	let s = "测试一下，Think diff";

	let chars: Vec<char> = s.chars().collect();


	let ss: String = chars.into_iter().collect();
	println!("{:?}", ss);


}
