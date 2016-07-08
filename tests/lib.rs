extern crate chinese_pinyin;

#[test]
fn test_t() {
	let args = chinese_pinyin::Args::default();

	assert_eq!("zhong guo", chinese_pinyin::t("中国", &args));
	assert_eq!("zhong guo english ri", chinese_pinyin::t("中国english日", &args));
	assert_eq!("shang hai very good o ye", chinese_pinyin::t("上海very good哦耶", &args));
}

#[test]
fn test_t_with_splitter() {
	let mut args = chinese_pinyin::Args::new();
	args.splitter = "-".to_string();

	assert_eq!("zhong-guo", chinese_pinyin::t("中国", &args));
	assert_eq!("guang-zhou", chinese_pinyin::t("广州", &args));
	assert_eq!("shang-hai", chinese_pinyin::t("上海", &args));
}

#[test]
fn test_t_with_tone() {
	let mut args = chinese_pinyin::Args::new();
	args.tone = true;

	assert_eq!("zhong1 guo2", chinese_pinyin::t("中国", &args));
	assert_eq!("shang4 hai3", chinese_pinyin::t("上海", &args));
}

#[test]
fn test_t_with_camelcase() {
	let mut args = chinese_pinyin::Args::new();

	args.camel = true;
	assert_eq!("Zhong Guo", chinese_pinyin::t("中国", &args));

	args.tone = true;
	assert_eq!("Zhong1 Guo2", chinese_pinyin::t("中国", &args));

	args.splitter = "-".to_string();
	assert_eq!("Shang4-Hai3", chinese_pinyin::t("上海", &args));

	args.tone = false;
	assert_eq!("Zhong-Guo", chinese_pinyin::t("中国", &args));
	assert_eq!("Guang-Zhou", chinese_pinyin::t("广州", &args));
	assert_eq!("Shang-Hai", chinese_pinyin::t("上海", &args));
}

#[test]
fn test_t_with_chinese_punctuation() {
	let mut args = chinese_pinyin::Args::new();
	args.splitter = "-".to_string();

  	assert_eq!("ce-shi-yi-xia，Think diff", chinese_pinyin::t("测试一下，Think diff", &args));
}

#[test]
fn test_t_with_tonemarks() {
	let mut args = chinese_pinyin::Args::new();
	args.tonemarks = true;

	assert_eq!("zhōng guó", chinese_pinyin::t("中国", &args));
	assert_eq!("běi jīng", chinese_pinyin::t("北京", &args));
}
