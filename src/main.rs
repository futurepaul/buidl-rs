#[macro_use]
extern crate arrayref;
extern crate rand;
extern crate rustc_serialize as serialize;
extern crate secp256k1;
extern crate sha2;

use rand::thread_rng;
use secp256k1::{key, Message, Secp256k1, Signature};
use sha2::{Digest, Sha256};
//use std::convert::AsRef;

#[derive(Debug)]
struct Transfer {
  signature: Signature,
  public_key: key::PublicKey,
}

#[derive(Debug)]
struct ECDSACoin {
  transfers: Vec<Transfer>,
}

fn byteify(thing: &[u8]) -> [u8; 32] {
  array_ref!(thing, 0, 32).clone()
}

fn hash_256(key: key::PublicKey) -> [u8; 32] {
  let mut sha2 = Sha256::new();

  // write input message
  sha2.input(&key.serialize().as_ref());

  // output
  let result = sha2.result();

  byteify(result.as_slice())
}

impl ECDSACoin {
  fn issue(public_key: key::PublicKey, bank_private_key: key::SecretKey) -> ECDSACoin {
    let secp = Secp256k1::new();
    public_key.serialize();

    let message = Message::from_slice(&hash_256(public_key)).expect("32 bytes");
    let signature = secp.sign(&message, &bank_private_key);

    let transfer = Transfer {
      signature: signature,
      public_key: public_key,
    };
    ECDSACoin {
      transfers: vec![transfer],
    }
  }
}

fn main() {
  let secp = Secp256k1::new();
  let mut rng = thread_rng();

  //Create the public / private keys
  let (bank_secret_key, bank_public_key) = secp.generate_keypair(&mut rng);
  let (alice_secret_key, alice_public_key) = secp.generate_keypair(&mut rng);
  let (bob_secret_key, bob_public_key) = secp.generate_keypair(&mut rng);

  let coin = ECDSACoin::issue(alice_public_key, bank_secret_key);

  println!("{:?}", coin);

  // let sig = secp.sign(&message, &alice_secret_key);
  // assert!(secp.verify(&message, &sig, &alice_public_key).is_ok());
}
