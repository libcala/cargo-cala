// bin/parse.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use toml;
use toml::Value;

use file;
use program;

// #[derive(Deserialize, Debug)]
pub struct CargoToml {
	pub domain: String,
	pub subdomain: String,
	pub version: String,
	pub version_number: u32,
	pub dependencies: Vec<(String, String)>,
}

pub fn get(a: &toml::Value, vname: &str) -> toml::Value {
	match a.get(vname) {
		Some(v) => v.clone(),
		None => {
			program::exit(&format!("Couldn't find key {}!", vname));
			unreachable!()
		},
	}
}

/// Spaces -> Underscores, Remove Punctuation and spell out numbers.
pub fn simplify(input: String) -> String {
	let https = "https://";
	let http = "http://";

	let input = if input.starts_with(https) {
		&input[https.len()..]
	} else if input.starts_with(http) {
		&input[http.len()..]
	} else {
		&input[..]
	}.to_string();

	let domain = {
		let mut domain : Vec<&str> = input.rsplit('/').collect();

		domain.pop().unwrap()
	};

	let mut output = String::new();
	let input : Vec<&str> = domain.rsplit('.').collect();

	for x in input {
		output.push_str(x);
		output.push('.');
	}

	output.pop();

	output
}

pub fn version_num(input: String) -> u32 {
	let v: Vec<&str> = input.split('.').collect();

	let major = v[0].parse::<u32>().unwrap();
	let minor = v[1].parse::<u32>().unwrap();
	let debug = v[2].parse::<u32>().unwrap();

	debug + (minor * 1_000) + (major * 1_000_000)
}

pub fn load(file: &str) -> toml::Value {
	let byte_vec = file::load(file);
	let file_dat : String = match String::from_utf8(byte_vec) {
		Ok(v) => v,
		Err(_) => {
			program::exit("Cargo.toml is not UTF-8!");
			unreachable!()
		}
	};

	let r : Result<toml::Value, _> = file_dat.parse();
	match r {
		Ok(v) => v,
		Err(e) => {
			println!("{}", e);
			program::exit("Cargo.toml is not TOML!");
			unreachable!()
		}
	}
}

pub fn execute() -> CargoToml {
	let ct = load("Cargo.toml");

	let package = get(&ct, "package");

	let name = if let Value::String(name) = get(&package, "name") {
		name
	} else {
		program::exit("package/name is not a string!");
		unreachable!()
	};

	let version = if let Value::String(version) = get(&package, "version") {
		version
	} else {
		program::exit("package/version is not a string!");
		unreachable!()
	};

	let org_name = if let Value::String(org) = get(&package, "homepage") {
		org
	} else {
		program::exit("package/homepage is not a string!");
		unreachable!()
	};

	let deps = get(&ct, "dependencies");

	let dependencies = if let Some(deps_arr) = deps.as_table() {
		let mut deps = Vec::new();
		for elem in deps_arr {
			let dep = if let Value::String(ref dep) = *elem.1 {
				dep.to_string()
			} else if let Value::Table(ref dep) = *elem.1 {
				let mut string = String::new();

				string.push_str("{ ");
				for elem in dep {
					string.push_str(&format!("{} = {}",
						elem.0, elem.1));
				}
				string.push_str(" }");

				string
			} else {
				println!("{}", elem.1);
				program::exit("dependency is not a string!");
				unreachable!()
			};

			deps.push((elem.0.to_string(), dep))
		}
		deps
	} else {
		program::exit("dependencies is not an array!");
		unreachable!()
	};

	let domain = simplify(org_name.clone());
	let subdomain = name.clone();
	let version_number = version_num(version.clone());

//	println!("Cargo.toml: {{ {:?} }}", ct);
//	println!("Name: {}", name);
//	println!("Version: {}", version);
	println!("Organization: {}", org_name);
	println!("Domain: {}.{}", domain, subdomain);
//	println!("Version Number: {}", version_number);

	CargoToml {
		domain, subdomain, version, version_number, dependencies
	}
}
