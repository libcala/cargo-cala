// bin/resources.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use toml::Value;

use file;
use parse;
use program;

const LANG_EN : &'static str = "res/text_en.toml";

pub struct English {
	pub us: Option<String>,
	pub gb: Option<String>,
	// TODO: Other Dialects
}

pub struct Spanish {
	pub es: Option<(String, String)>,
	pub mx: Option<(String, String)>,
	// TODO: Other Dialects
}

pub struct Lang {
	pub en: Option<((String, String), English)>,
	pub es: Option<((String, String), Spanish)>,
}

fn get_name(lang: &str) -> (String, String) {
	let en = parse::load(LANG_EN);

	let name = if let Value::String(name) = parse::get(&en, "name") {
		name
	} else {
		program::exit(&format!("{}: name is not a string!", lang));
		unreachable!()
	};

	let comment = if let Value::String(sd) = parse::get(&en, "comment") {
		sd
	} else {
		program::exit(&format!("{}: comment is not a string!", lang));
		unreachable!()
	};

	(name, comment)
}

pub fn execute(cargo_toml: &parse::CargoToml) -> Lang {
	let mut lang = Lang {
		en: None,
		es: None,
	};

	let resources = "target/resources/".to_string();

	// Find Translations
	{
		if file::get_exists(LANG_EN) {
			let english = English {
				us: None,
				gb: None,
			};

			lang.en = Some((get_name(LANG_EN), english));
		}
	}

	// Write title.rs
	{
		let mut data = "{\
			let lang = env!(\"LANG\");\
			let lang_code = &lang[0..2];\
			let area_code = &lang[3..5];\
			\
			match lang_code {\
		".to_string();

		if let Some(ref en) = lang.en {
			data.push_str("\"en\" => match area_code {");
			// TODO: Dialects
			data.push_str(&format!("_ => \"{}\" }},", en.0 .0));
		}

		// Close Match, Close Block
		data.push_str(&format!("_=>\"{}\"}}}}", &cargo_toml.subdomain));

		let file = {
			let mut x = resources.clone();

			x.push_str("title.rs");
			x
		};

		file::save(&file, data.as_bytes());
	}

	lang
}
