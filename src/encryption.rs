use crate::{Config, EncryptionKey};
use data_encoding::BASE64;
use sodiumoxide::crypto::box_::*;
use sodiumoxide::crypto::sealedbox;
use std::collections::HashMap;
use std::env;
use std::str;

fn resolve_env_key(key: &str) -> String {
    return if key.starts_with('$') {
        // Remove the $ from the environment variable:
        let mut env_key = key.chars();
        env_key.next();
        env::var(env_key.as_str()).unwrap_or_else(|_| "".to_string())
    } else {
        key.to_string()
    };
}

pub struct Encryption<'a> {
    pub config: &'a Config,
}

impl Encryption<'_> {
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

        let public_key = resolve_env_key(&keys.public_key);
        let secret_key = resolve_env_key(&keys.secret_key);

        Ok(EncryptionKey {
            public_key,
            secret_key,
        })
    }

    fn get_sec_key(&self, key: &str) -> Result<SecretKey, &'static str> {
        let sec_key = self.resolve_keys(key)?.secret_key;

        // sodiumoxide needs fixed-length arrays, not slices
        let seckey_decoded = BASE64.decode(sec_key.as_bytes()).unwrap_or_default();
        if seckey_decoded.len() != SECRETKEYBYTES {
            return Err("The secret key did not match the expected format.");
        }

        let mut seckey_bytes = [0u8; SECRETKEYBYTES];
        seckey_bytes[..SECRETKEYBYTES].clone_from_slice(&seckey_decoded[..SECRETKEYBYTES]);

        Ok(SecretKey(seckey_bytes))
    }

    fn get_pub_key(&self, key: &str) -> Result<PublicKey, &'static str> {
        let pub_key = self.resolve_keys(key)?.public_key;

        // sodiumoxide needs fixed-length arrays, not slices
        let pubkey_decoded = BASE64.decode(pub_key.as_bytes()).unwrap_or_default();
        if pubkey_decoded.len() != PUBLICKEYBYTES {
            return Err("The public key did not match the expected format.");
        }

        let mut pubkey_bytes = [0u8; PUBLICKEYBYTES];
        pubkey_bytes[..PUBLICKEYBYTES].clone_from_slice(&pubkey_decoded[..PUBLICKEYBYTES]);

        Ok(PublicKey(pubkey_bytes))
    }
}
