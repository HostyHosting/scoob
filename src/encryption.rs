use crate::config::{Config, EncryptionKey};
use data_encoding::BASE64;
use data_encoding::HEXUPPER_PERMISSIVE;
use rusoto_secretsmanager::GetSecretValueRequest;
use rusoto_secretsmanager::SecretsManager;
use rusoto_secretsmanager::SecretsManagerClient;
use sodiumoxide::crypto::box_::*;
use sodiumoxide::crypto::sealedbox;
use std::collections::HashMap;
use std::env;
use std::str;

fn resolve_key(key: &str) -> String {
    if key.contains('$') {
        let mut parts = key.splitn(2, '$');
        let key_type = parts.next();
        let key_data = parts.next();

        if key_type == Some("env") || key_type == Some("") || key_type == None {
            let value = key_data.and_then(|k| env::var(k).ok());
            return value.unwrap_or_else(|| "".into());
        } else if key_type == Some("awsSecretsManager") {
            if key_data == None {
                return "".into();
            }

            let client = SecretsManagerClient::new(rusoto_core::Region::UsEast1);
            let request = GetSecretValueRequest {
                secret_id: key_data.unwrap().into(),
                ..Default::default()
            };

            let runtime = tokio::runtime::Runtime::new().unwrap();

            let result = runtime.block_on(client.get_secret_value(request));

            if let Ok(response) = result {
                let secret = response.secret_string.unwrap_or_else(|| "".into());
                return secret;
            }

            return "".into();
        }
    }

    key.to_string()
}

pub struct Encryption<'a> {
    pub config: &'a Config,
}

impl Encryption<'_> {
    pub fn gen_keypair() -> (String, String) {
        let (public_key, secret_key) = gen_keypair();
        (
            BASE64.encode((public_key).0.as_ref()),
            BASE64.encode((secret_key).0.as_ref()),
        )
    }

    pub fn encrypt_configuration(&self, new_config: &Config) -> Result<Config, &'static str> {
        let mut encrypted_configuration: HashMap<String, String> = HashMap::new();

        let new_encrypter = Encryption { config: new_config };

        for (key, value) in new_config.configuration.iter() {
            match value.as_str() {
                // Encrypted value that has not changed. We just use the previous value:
                "<encrypted>" => {
                    let previous_value = self.config.configuration.get(key);
                    if previous_value.is_none() {
                        return Err("Encrypted values cannot be moved or renamed.");
                    }

                    encrypted_configuration.insert(
                        key.to_string(),
                        previous_value
                            .expect("Previous value is missing.")
                            .to_string(),
                    )
                }
                // New value:
                _ => encrypted_configuration
                    .insert(key.to_string(), new_encrypter.encrypt(key, value)?),
            };
        }

        Ok(Config {
            configuration: encrypted_configuration,
            keys: new_config.keys.clone(),
        })
    }

    pub fn encrypt(&self, key: &str, value: &str) -> Result<String, &'static str> {
        let public_key = self.get_pub_key(key)?;

        let message = sealedbox::seal(value.as_bytes(), &public_key);
        Ok(BASE64.encode(&message))
    }

    pub fn decrypt(&self, key: &str, value: &str) -> Result<String, &'static str> {
        let public_key = self.get_pub_key(key)?;
        let secret_key = self.get_sec_key(key)?;

        let decoded = match BASE64.decode(value.as_bytes()) {
            Ok(val) => val,
            Err(_) => return Err("Secret was not base64 encoded."),
        };

        let decrypted = match sealedbox::open(&decoded, &public_key, &secret_key) {
            Ok(val) => val,
            Err(_) => return Err("Failed to decrypt secret."),
        };

        Ok(match str::from_utf8(&decrypted) {
            Ok(val) => val.to_string(),
            Err(_) => return Err("Secret was not utf8 encoded"),
        })
    }

    fn resolve_keys(&self, key: &str) -> Result<EncryptionKey, &'static str> {
        let keys = match self
            .config
            .keys
            .get(key)
            .or_else(|| self.config.keys.get("*"))
        {
            Some(val) => val,
            None => return Err("Missing encryption keys."),
        };

        let public_key = resolve_key(&keys.public_key);
        let secret_key = resolve_key(&keys.secret_key);

        Ok(EncryptionKey {
            public_key,
            secret_key,
        })
    }

    fn get_sec_key(&self, key: &str) -> Result<SecretKey, &'static str> {
        let sec_key = self.resolve_keys(key)?.secret_key;
        let seckey_decoded: Vec<u8>;

        if sec_key.as_bytes().len() == 64 {
            seckey_decoded = HEXUPPER_PERMISSIVE
                .decode(sec_key.as_bytes())
                .unwrap_or_default();
        } else {
            seckey_decoded = BASE64.decode(sec_key.as_bytes()).unwrap_or_default();
        }

        if seckey_decoded.len() != SECRETKEYBYTES {
            return Err("The secret key did not match the expected format.");
        }

        let mut seckey_bytes = [0u8; SECRETKEYBYTES];
        seckey_bytes[..SECRETKEYBYTES].clone_from_slice(&seckey_decoded[..SECRETKEYBYTES]);

        Ok(SecretKey(seckey_bytes))
    }

    fn get_pub_key(&self, key: &str) -> Result<PublicKey, &'static str> {
        let pub_key = self.resolve_keys(key)?.public_key;
        let pubkey_decoded: Vec<u8>;

        if pub_key.as_bytes().len() == 64 {
            pubkey_decoded = HEXUPPER_PERMISSIVE
                .decode(pub_key.as_bytes())
                .unwrap_or_default();
        } else {
            pubkey_decoded = BASE64.decode(pub_key.as_bytes()).unwrap_or_default();
        }

        if pubkey_decoded.len() != PUBLICKEYBYTES {
            return Err("The public key did not match the expected format.");
        }

        let mut pubkey_bytes = [0u8; PUBLICKEYBYTES];
        pubkey_bytes[..PUBLICKEYBYTES].clone_from_slice(&pubkey_decoded[..PUBLICKEYBYTES]);

        Ok(PublicKey(pubkey_bytes))
    }
}

// NOTE: Tests here have to disable exec, otherwise it would replace the test process itself.
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_encrypt_configuration() -> Result<(), &'static str> {
        let original_config = Config::default();
        let mut new_config = original_config.clone();
        let enc = Encryption {
            config: &original_config,
        };

        let new_value = "this is a new value for the new key";

        new_config
            .configuration
            .insert("NEW_KEY".to_string(), new_value.to_string());

        let encrypted_config = enc.encrypt_configuration(&new_config)?;

        let encrypted_key = encrypted_config.configuration.get(&"NEW_KEY".to_string());
        assert!(encrypted_key.is_some());
        assert_ne!(encrypted_key.unwrap(), new_value);

        assert_eq!(
            Encryption {
                config: &encrypted_config
            }
            .decrypt("NEW_KEY", encrypted_key.unwrap())
            .unwrap(),
            new_value
        );

        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt() -> Result<(), &'static str> {
        let mut path = env::current_dir().unwrap();
        path.push("test");
        path.push("secrets.yml");
        let config = Config::get(&path);
        let enc = Encryption { config: &config };
        let raw_value = "string to encrypt";
        let encrypted = enc.encrypt("test", raw_value)?;
        let decrypted = enc.decrypt("test", &encrypted)?;
        assert_eq!(decrypted, raw_value);
        Ok(())
    }

    #[test]
    fn test_env_key() {
        // TODO: Write test for env variable keys.
    }

    #[test]
    fn test_missing_keys() {
        let mut config = Config::default();
        config.keys.remove("*").unwrap();
        let enc = Encryption { config: &config };
        assert!(enc.encrypt("key", "value").is_err());
    }
}
