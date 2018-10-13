#![allow(dead_code)]
#![allow(unused_variables)]
#[macro_use]
extern crate arrayref;
extern crate rand;
extern crate rustc_serialize as serialize;
extern crate secp256k1;
extern crate sha2;
extern crate uuid;

mod bank;
mod tx;
mod util;

use uuid::Uuid;

use rand::thread_rng;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};
use std::collections::HashMap;

use crate::bank::Bank;
use crate::tx::{Tx, TxIn, TxOut};
use crate::util::hash_256_from_key;

#[derive(Debug)]
struct Transfer {
  message: Message,
  signature: Signature,
  public_key: PublicKey,
}

#[derive(Debug)]
struct ECDSACoin {
  transfers: Vec<Transfer>,
}

// macro_rules! hex {
//   ($hex:expr) => {{
//     let mut vec = Vec::new();
//     let mut b = 0;
//     for (idx, c) in $hex.as_bytes().iter().enumerate() {
//       b <<= 4;
//       match *c {
//         b'A'...b'F' => b |= c - b'A' + 10,
//         b'a'...b'f' => b |= c - b'a' + 10,
//         b'0'...b'9' => b |= c - b'0',
//         _ => panic!("Bad hex"),
//       }
//       if (idx & 1) == 1 {
//         vec.push(b);
//         b = 0;
//       }
//     }
//     vec
//   }};
// }

impl ECDSACoin {
  fn issue(public_key: PublicKey, bank_private_key: SecretKey) -> ECDSACoin {
    let secp = Secp256k1::new();
    // public_key.serialize();

    let message = Message::from_slice(&hash_256_from_key(public_key)).expect("32 bytes");
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

fn validate(secp: Secp256k1<secp256k1::All>, coin: ECDSACoin, bank_public_key: PublicKey) -> bool {
  let transfer = &coin.transfers[0];
  let message = Message::from_slice(&hash_256_from_key(transfer.public_key)).expect("32 bytes");
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

  //Make a bank
  let mut bank = Bank {
    utxo: HashMap::new(),
    secp: secp,
  };

  //Mutate the bank by issuing coins to alice
  let coinbase = bank.issue(1000, alice_public_key);

  let tx_ins = vec![TxIn {
    tx_id: coinbase.id,
    index: 0,
    signature: None,
  }];

  let tx_id = Uuid::new_v4();

  //Build the transaction. Alice gives 10 to bob, 990 to herself.
  let tx_outs = vec![
    TxOut {
      tx_id: tx_id,
      index: 0,
      amount: 10,
      public_key: bob_public_key,
    },
    TxOut {
      tx_id: tx_id,
      index: 1,
      amount: 990,
      public_key: alice_public_key,
    },
  ];

  let mut alice_to_bob = Tx {
    id: tx_id,
    tx_ins: tx_ins,
    tx_outs: tx_outs,
  };

  //Sign it! (still haven't crashed)
  alice_to_bob.sign_input(0, alice_secret_key);

  //println!("alice_to_bob tx = {:?}", alice_to_bob);

  //When the bank tries to verify, it doesn't like the signature or something
  bank.handle_tx(alice_to_bob);
}
