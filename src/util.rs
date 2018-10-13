use rand::{thread_rng, Rng};
use secp256k1::{Message, PublicKey};
use sha2::{Digest, Sha256};

pub fn messageify() -> Message {
  Message::from_slice(&rando_msg()).expect("32 bytes")
}

fn rando_msg() -> [u8; 32] {
  let mut msg = [0u8; 32];
  thread_rng().fill_bytes(&mut msg);
  msg
}

pub fn hash_256_from_string(message: String) -> [u8; 32] {
  let mut sha2 = Sha256::new();

  // write input message
  sha2.input(&message);

  // output
  let result = sha2.result();

  //am I slicing off part of this?
  byteify(result.as_slice())
}

pub fn hash_256_from_key(key: PublicKey) -> [u8; 32] {
  let mut sha2 = Sha256::new();

  // write input message
  sha2.input(&key.serialize().as_ref());

  // output
  let result = sha2.result();

  //am I slicing off part of this?
  byteify(result.as_slice())
}

fn byteify(thing: &[u8]) -> [u8; 32] {
  array_ref!(thing, 0, 32).clone()
}
