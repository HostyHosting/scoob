use data_encoding::BASE64;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::gen_keypair;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptionKey {
	#[serde(rename = "publicKey")]
	pub public_key: String,
	#[serde(rename = "secretKey")]
	pub secret_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
	pub configuration: HashMap<String, String>,
	pub keys: HashMap<String, EncryptionKey>,
}

pub struct Config {}

impl Config {
	pub fn get_config(path: &PathBuf) -> ConfigFile {
		let content = std::fs::read_to_string(path).expect("Could not read secrets file.");
		let config: ConfigFile = serde_yaml::from_str(&content).expect("Do not break pls.");
		println!("{:?}", config);
		config
	}

	pub fn default_config() -> ConfigFile {
		let (public_key, secret_key) = gen_keypair();

		let mut default_config: HashMap<String, String> = HashMap::new();
		let mut default_keys = HashMap::new();

		default_config.insert(
			"EXAMPLE_KEY".to_string(),
			"some value that should be encrypted".to_string(),
		);

		default_keys.insert(
			"*".to_string(),
			EncryptionKey {
				public_key: BASE64.encode((public_key).0.as_ref()),
				secret_key: BASE64.encode((secret_key).0.as_ref()),
			},
		);

		ConfigFile {
			configuration: default_config,
			keys: default_keys,
		}
	}
}
