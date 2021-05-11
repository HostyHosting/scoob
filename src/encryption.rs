use crate::ConfigFile;
use data_encoding::BASE64;
use sodiumoxide::crypto::box_::*;
use sodiumoxide::crypto::sealedbox;
use std::str;

pub struct Encryption {
	pub config: ConfigFile,
}

impl Encryption {
	pub fn encrypt(&self, key: &String, value: &String) -> String {
		let public_key = self.get_pub_key(key);

		let message = sealedbox::seal(value.as_bytes(), &public_key);
		BASE64.encode(&message)
	}

	pub fn decrypt(&self, key: &String, value: &String) -> String {
		let public_key = self.get_pub_key(key);
		let secret_key = self.get_sec_key(key);

		str::from_utf8(&sealedbox::open(value.as_bytes(), &public_key, &secret_key).unwrap())
			.unwrap()
			.to_string()
	}

	fn get_sec_key(&self, new_key: &String) -> SecretKey {
		let sec_key = &self
			.config
			.keys
			.get(new_key)
			.unwrap_or(self.config.keys.get("*").unwrap())
			.secret_key;

		// sodiumoxide needs fixed-length arrays, not slices
		let seckey_decoded = BASE64.decode(sec_key.as_bytes()).unwrap();
		assert!(seckey_decoded.len() == SECRETKEYBYTES);

		let mut seckey_bytes = [0u8; SECRETKEYBYTES];
		for i in 0..SECRETKEYBYTES {
			seckey_bytes[i] = seckey_decoded[i];
		}

		SecretKey(seckey_bytes)
	}

	fn get_pub_key(&self, new_key: &String) -> PublicKey {
		let pub_key = &self
			.config
			.keys
			.get(new_key)
			.unwrap_or(self.config.keys.get("*").unwrap())
			.public_key;

		// sodiumoxide needs fixed-length arrays, not slices
		let pubkey_decoded = BASE64.decode(pub_key.as_bytes()).unwrap();
		assert!(pubkey_decoded.len() == PUBLICKEYBYTES);

		let mut pubkey_bytes = [0u8; PUBLICKEYBYTES];
		for i in 0..PUBLICKEYBYTES {
			pubkey_bytes[i] = pubkey_decoded[i];
		}

		PublicKey(pubkey_bytes)
	}
}
