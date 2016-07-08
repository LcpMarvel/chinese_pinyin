extern crate phf;

use std::iter::FromIterator;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

const VOWELS: [&'static str; 6] = ["a", "o", "e", "i", "u", "v"];

const TONE_MARKS: [[&'static str; 5]; 6] = [
	["a", "ā", "á", "ǎ", "à"],
	["o", "ō", "ó", "ǒ", "ò"],
	["e", "ē", "é", "ě", "è"],
	["i", "ī", "í", "ǐ", "ì"],
	["u", "ū", "ú", "ǔ", "ù"],
	["ü", "ǖ", "ǘ", "ǚ", "ǜ"]
];

#[derive(Debug)]
pub struct Args<'a> {
	pub splitter: &'a str,
	pub tonemarks: bool,
	pub tone: bool,
	pub camel: bool
}

impl<'a> Default for Args<'a> {
	fn default() -> Args<'a> {
		Args {
			splitter: " ",
			tonemarks: false,
			tone: false,
			camel: false
		}
	}
}

fn capitalize(string: &str) -> String {
	let first_character = string.chars().nth(0).unwrap()
							   .to_string().to_uppercase();

	let tail_characters = &string[1..string.len()];

	format!("{}{tail}", first_character, tail = tail_characters)
}

fn truncated_pinyin(pinyin: String) -> String {
	let mut cloned_pinyin = pinyin.clone();

	cloned_pinyin.truncate(pinyin.len() - 1);

	cloned_pinyin
}

fn find_tone_index(string: &str) -> i32 {
	let index = string.len() - 1;

	if let Ok(value) = string.chars().nth(index).unwrap().to_string().parse::<i32>() {
		value
	} else {
		0
	}
}

fn replace_vowels(string: &str, tone_index: i32) -> String {
	for (index, vowel) in VOWELS.iter().enumerate() {
		if let Some(_) = string.find(vowel) {
			return string.replace(
				vowel, TONE_MARKS[index][tone_index as usize]
			)
		}
	}

	string.to_string()
}

pub fn translate<'a>(characters: &str, args: &Args<'a>) -> String {
	// let splitter = args.splitter;

	let mut results: Vec<String> = Vec::new();
	// let mut valid_chinese = true;

	for c in characters.chars() {
		let string_c = c.to_string();

		if let Some(pinyin) = DATA.get(&*string_c) {
			let pinyin_string = pinyin.to_string();
			let tone_index = find_tone_index(&pinyin_string);
			let without_tone_pinyin_string = truncated_pinyin(pinyin_string);

			// if !valid_chinese {
			// 	results.push(splitter);
			// }
			// valid_chinese = true;

			if args.tone && args.tonemarks {           // 需要声调
				results.push(
					replace_vowels(&without_tone_pinyin_string, tone_index)
				);
			} else if !args.tone {                     // 不需要声调
				results.push(without_tone_pinyin_string);
			} else if args.camel {                     // 需要首字母大写
				results.push(capitalize(&pinyin));
			}
		} else if c.is_digit(10) {
			results.push(string_c);
		}
	}

	String::from_iter(results)
}

pub fn t<'a>(characters: &str, args: &Args<'a>) -> String {
	translate(characters, args)
}

#[test]
fn it_has_default_args() {
	let args = Args::default();

	assert_eq!(" ", args.splitter);
	assert_eq!(false, args.tonemarks);
	assert_eq!(false, args.tone);
	assert_eq!(false, args.camel);
}

#[test]
fn it_capitalizes_string() {
	let string = "up";

	assert_eq!("Up".to_string(), capitalize(&string));
}

#[test]
fn it_finds_tone_index() {
	let string = "tai2";

	assert_eq!(2, find_tone_index(&string));
}

#[test]
fn it_replaces_vowels_for_pinyin() {
	let string = "tai";

	assert_eq!("tái", replace_vowels(&string, 2));
}
