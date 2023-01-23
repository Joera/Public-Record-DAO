use crate::keccak::Keccak256;
use crate::memory::{store};

use std::str;
use std::convert::TryInto;
use libsecp256k1::{SecretKey, PublicKey};
use ethers_core::types::{ PrivateKey }; // Bytes
use rand::rngs::OsRng;
use hex as hex;
use serde::{Deserialize};


pub fn new() -> String {

  let secret_key = SecretKey::random(&mut OsRng);
  let public_key = PublicKey::from_secret_key(&secret_key);
  let secret_string = format_secret_key(secret_key);
  let address = format_address(public_key);
    
  store(&address, &secret_string);
  address
}

pub fn new_dev() -> Vec<String> {

  let secret_key = SecretKey::random(&mut OsRng);
  let public_key = PublicKey::from_secret_key(&secret_key);
  let secret_string = format_secret_key(secret_key);
  let address = format_address(public_key);
    
  store(&address, &secret_string);
  vec!(address, secret_string)
}

pub fn from_secret(secret: String) -> Vec<String> {

  let private_key: PrivateKey = secret.parse().unwrap();
  let secret: [u8;32] = private_key.serialize();
  let secret_key: SecretKey = SecretKey::parse(&secret).unwrap();

  let public_key = PublicKey::from_secret_key(&secret_key);
  let secret_string = format_secret_key(secret_key);
  let address = format_address(public_key);
    
  store(&address, &secret_string);
  vec!(address, secret_string)
}

fn format_address(public_key: PublicKey) -> String {

  let hash = public_key.serialize()[1..65].keccak256();
  let trimmed: [u8; 20] = hash[12..32].try_into().unwrap();
  format!("0x{}", hex::encode(trimmed))
}

fn format_secret_key(skey : SecretKey) -> String {
  hex::encode(skey.serialize())
}
