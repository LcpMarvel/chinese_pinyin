extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write, BufRead};
use std::path::Path;

fn read_file(path: &std::path::Path) -> Result<BufReader<File>, io::Error> {
	let f = try!(File::open(path));

	Ok(BufReader::new(f))
}

fn quote_string(string: &str) -> String {
	format!("\"{}\"", &string)
}

fn main() {
	let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
	let mut file = BufWriter::new(File::create(&path).unwrap());

	let reader = read_file(Path::new("./data/pinyin-utf8.dat")).unwrap();

	write!(&mut file, "static DATA: phf::Map<&'static str, &'static str> = ");

	let mut builder = phf_codegen::Map::new();

	for line in reader.lines() {
		let unwrapped_line = line.unwrap();

		let mut chars = unwrapped_line.trim().split_whitespace();

		let chinese = chars.next().unwrap();
		let pinyin = chars.next().unwrap();

		builder.entry(
			chinese.to_string(), &quote_string(pinyin)
		);
	}

	builder.build(&mut file).unwrap();

	write!(&mut file, ";\n").unwrap();
}
