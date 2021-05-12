use data_encoding::BASE64;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::gen_keypair;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncryptionKey {
	#[serde(rename = "publicKey")]
	pub public_key: String,
	#[serde(rename = "secretKey")]
	pub secret_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigFile {
	pub configuration: HashMap<String, String>,
	pub keys: HashMap<String, EncryptionKey>,
}

impl ConfigFile {
	pub fn with_placeholders(&self) -> ConfigFile {
		let mut placeholder_configuration: HashMap<String, String> = HashMap::new();

		for key in self.configuration.keys() {
			placeholder_configuration.insert(key.to_string(), "<encrypted>".to_string());
		}

		ConfigFile {
			configuration: placeholder_configuration,
			keys: self.keys.clone(),
		}
	}
}

pub struct Config {}

impl Config {
	pub fn exists(path: &PathBuf) -> bool {
		let result = std::fs::read_to_string(path);
		match result {
			Ok(_) => true,
			Err(_) => false,
		}
	}

	pub fn get_config(path: &PathBuf) -> ConfigFile {
		let result = std::fs::read_to_string(path);
		match result {
			Ok(content) => {
				// TODO: Invalid structure of configuration file
				serde_yaml::from_str(&content).unwrap()
			}
			Err(_) => Config::default_config(),
		}
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
