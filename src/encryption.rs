use crate::Config;
use data_encoding::BASE64;
use sodiumoxide::crypto::box_::*;
use sodiumoxide::crypto::sealedbox;
use std::str;

fn get_pub_key(config: &Config, new_key: &String) -> PublicKey {
	let pubKey = config
		.keys
		.iter()
		.find(|&key| key.for_keys.iter().any(|for_key| for_key == new_key))
		.unwrap()
		.config
		.get("publicKey")
		.unwrap();

	// sodiumoxide needs fixed-length arrays,
	// not slices
	let pubkey_decoded = BASE64.decode(pubKey.as_bytes()).unwrap();
	assert!(pubkey_decoded.len() == PUBLICKEYBYTES);

	let mut pubkey_bytes = [0u8; PUBLICKEYBYTES];
	for i in 0..PUBLICKEYBYTES {
		pubkey_bytes[i] = pubkey_decoded[i];
	}

	PublicKey(pubkey_bytes)
}

pub fn encrypt(config: &Config, new_key: &String, new_value: &String) -> String {
	let key = get_pub_key(config, new_key);

	let message = sealedbox::seal(new_value.as_bytes(), &key);
	BASE64.encode(&message)
}

pub fn decrypt(value: &String) {
	// let key = secretbox::gen_key();
	// let nonce = secretbox::gen_nonce();
	// let plaintext = b"some data";
	// let ciphertext = secretbox::seal(plaintext, &nonce, &key);
	// let their_plaintext = secretbox::open(&ciphertext, &nonce, &key).unwrap();
	// println!("{:?}", ciphertext);
}
