#[macro_use]
extern crate arrayref;
extern crate rand;
extern crate rustc_serialize as serialize;
extern crate secp256k1;
extern crate sha2;

use rand::{thread_rng, Rng};
use secp256k1::{key, Message, Secp256k1, Signature};
use sha2::{Digest, Sha256};
//use std::convert::AsRef;

#[derive(Debug)]
struct Transfer {
  message: Message,
  signature: Signature,
  public_key: key::PublicKey,
}

#[derive(Debug)]
struct ECDSACoin {
  transfers: Vec<Transfer>,
}

macro_rules! hex {
  ($hex:expr) => {{
    let mut vec = Vec::new();
    let mut b = 0;
    for (idx, c) in $hex.as_bytes().iter().enumerate() {
      b <<= 4;
      match *c {
        b'A'...b'F' => b |= c - b'A' + 10,
        b'a'...b'f' => b |= c - b'a' + 10,
        b'0'...b'9' => b |= c - b'0',
        _ => panic!("Bad hex"),
      }
      if (idx & 1) == 1 {
        vec.push(b);
        b = 0;
      }
    }
    vec
  }};
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

fn rando_msg() -> [u8; 32] {
  let mut msg = [0u8; 32];
  thread_rng().fill_bytes(&mut msg);
  msg
}

impl ECDSACoin {
  fn issue(public_key: key::PublicKey, bank_private_key: key::SecretKey) -> ECDSACoin {
    let secp = Secp256k1::new();
    // public_key.serialize();

    let message = Message::from_slice(&hash_256(public_key)).expect("32 bytes");
    // let message = Message::from_slice(&rando_msg()).expect("32 bytes");
    let signature = secp.sign(&message, &bank_private_key);

    let transfer = Transfer {
      message: message,
      signature: signature,
      public_key: public_key,
    };
    ECDSACoin {
      transfers: vec![transfer],
    }
  }
}

fn validate(
  secp: Secp256k1<secp256k1::All>,
  coin: ECDSACoin,
  bank_public_key: key::PublicKey,
) -> bool {
  let transfer = &coin.transfers[0];
  let message = Message::from_slice(&hash_256(transfer.public_key)).expect("32 bytes");
  secp
    .verify(&message, &transfer.signature, &bank_public_key)
    .is_ok()
}

fn main() {
  let secp = Secp256k1::new();
  let mut rng = thread_rng();

  //Create the public / private keys
  let (bank_secret_key, bank_public_key) = secp.generate_keypair(&mut rng);
  let (alice_secret_key, alice_public_key) = secp.generate_keypair(&mut rng);
  let (bob_secret_key, bob_public_key) = secp.generate_keypair(&mut rng);

  let coin = ECDSACoin::issue(alice_public_key, bank_secret_key);

  // println!("{:?}", coin);

  let message = Message::from_slice(&hash_256(alice_public_key)).expect("32 bytes");

  // let sig = secp.sign(&message, &alice_secret_key);
  // assert!(secp.verify(&message, &sig, &alice_public_key).is_ok());
  assert!(
    secp
      .verify(&message, &coin.transfers[0].signature, &bank_public_key)
      .is_ok()
  );

  println!("{:?}", validate(secp, coin, bank_public_key));
}
