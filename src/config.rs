use data_encoding::BASE64;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::gen_keypair;
use std::collections::HashMap;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncryptionKey {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "secretKey")]
    pub secret_key: String,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub configuration: HashMap<String, String>,
    pub keys: HashMap<String, EncryptionKey>,
}

impl Config {
    pub fn with_placeholders(&self) -> Config {
        let mut placeholder_configuration: HashMap<String, String> = HashMap::new();

        for key in self.configuration.keys() {
            placeholder_configuration.insert(key.to_string(), "<encrypted>".to_string());
        }

        Config {
            configuration: placeholder_configuration,
            keys: self.keys.clone(),
        }
    }

    pub fn exists(path: &Path) -> bool {
        let result = std::fs::read_to_string(path);
        result.is_ok()
    }

    pub fn get(path: &Path) -> Config {
        let result = std::fs::read_to_string(path);
        match result {
            Ok(content) => {
                // TODO: Invalid structure of configuration file
                serde_yaml::from_str(&content).unwrap()
            }
            Err(_) => Config::default(),
        }
    }

    pub fn default() -> Config {
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

        Config {
            configuration: default_config,
            keys: default_keys,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        let default_keys = config
            .keys
            .get("*")
            .expect("Missing default encryption keys");

        config
            .configuration
            .get("EXAMPLE_KEY")
            .expect("Default configuration should include example key.");

        assert!(default_keys.public_key.chars().count() > 0);
        assert!(default_keys.secret_key.chars().count() > 0);
    }

    #[test]
    fn test_exists() -> std::io::Result<()> {
        let mut path = env::current_dir()?;
        path.push("test");
        path.push("secrets.yml");
        assert_eq!(Config::exists(&path), true);
        path.pop();
        path.push("does-not-exist.yml");
        assert_eq!(Config::exists(&path), false);
        Ok(())
    }

    #[test]
    fn test_read_config() -> std::io::Result<()> {
        let mut path = env::current_dir()?;
        path.push("test");
        path.push("secrets.yml");
        let config = Config::get(&path);
        config
            .configuration
            .get("TEST_KEY")
            .expect("Should include test key");
        Ok(())
    }

    #[test]
    fn test_config_placeholders() {
        let mut config = Config::default();
        config
            .configuration
            .insert("TEST".to_string(), "A test value".to_string());
        let placeholder_config = config.with_placeholders();
        assert_eq!(
            placeholder_config
                .configuration
                .get("TEST")
                .expect("Did not find test key"),
            "<encrypted>"
        );
    }
}
