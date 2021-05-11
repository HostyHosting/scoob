use data_encoding::BASE64;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::gen_keypair;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptionKey {
	pub provider: String,
	#[serde(rename = "for")]
	pub for_keys: Vec<String>,
	pub config: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
	pub configuration: HashMap<String, String>,
	pub keys: Vec<EncryptionKey>,
}

pub fn get_config(path: &PathBuf) -> Config {
	let content = std::fs::read_to_string(path).expect("Could not read secrets file.");
	let config: Config = serde_yaml::from_str(&content).expect("Do not break pls.");
	println!("{:?}", config);
	config
}

pub fn default_config() -> Config {
	let mut provider_config: HashMap<String, String> = HashMap::new();

	let (public_key, secret_key) = gen_keypair();

	provider_config.insert(
		"publicKey".to_string(),
		BASE64.encode((public_key).0.as_ref()),
	);
	provider_config.insert(
		"secretKey".to_string(),
		BASE64.encode((secret_key).0.as_ref()),
	);

	Config {
		configuration: HashMap::new(),
		keys: vec![EncryptionKey {
			provider: "env".to_string(),
			for_keys: vec!['*'.to_string()],
			config: provider_config,
		}],
	}
}
