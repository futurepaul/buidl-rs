#[macro_use]
extern crate arrayref;
extern crate hex;
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
use secp256k1::Secp256k1;
use std::collections::HashMap;

use crate::bank::Bank;
use crate::tx::{Tx, TxIn, TxOut};

fn main() {
  let secp = Secp256k1::new();
  let mut rng = thread_rng();

  //Create the public / private keys
  let (_bank_secret_key, _bank_public_key) = secp.generate_keypair(&mut rng);
  let (alice_secret_key, alice_public_key) = secp.generate_keypair(&mut rng);
  let (_bob_secret_key, bob_public_key) = secp.generate_keypair(&mut rng);

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

  //Now we let the bank update the ledger
  bank.handle_tx(alice_to_bob);

  assert_eq!(990, bank.fetch_balance(&alice_public_key));
  assert_eq!(10, bank.fetch_balance(&bob_public_key));

  println!(
    "Alice has {} bankcoin, Bob has {} bankcoin. Good job.",
    bank.fetch_balance(&alice_public_key),
    bank.fetch_balance(&bob_public_key)
  )
}
