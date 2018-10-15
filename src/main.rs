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
  let (bob_secret_key, bob_public_key) = secp.generate_keypair(&mut rng);

  //Make a bank
  let mut bank = Bank {
    utxo: HashMap::new(),
    secp: secp,
  };

  //Mutate the bank by issuing coins to alice
  let coinbase = bank.issue(1000, alice_public_key);

  println!(
    "Alice has {} bankcoin, Bob has {} bankcoin. Good job.",
    bank.fetch_balance(&alice_public_key),
    bank.fetch_balance(&bob_public_key)
  );

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

  //Sign it!
  alice_to_bob.sign_input(0, alice_secret_key);

  //Now we trust the bank to update the ledger
  bank.handle_tx(alice_to_bob);

  assert_eq!(990, bank.fetch_balance(&alice_public_key));
  assert_eq!(10, bank.fetch_balance(&bob_public_key));

  println!(
    "Alice has {} bankcoin, Bob has {} bankcoin. Good job.",
    bank.fetch_balance(&alice_public_key),
    bank.fetch_balance(&bob_public_key)
  );

  //Alright let's see if Bob can move bankcoin
  let bob_tx_id = Uuid::new_v4();

  //BUG: if I use the ID from alice's tx, it wipes out alice's balance
  let bob_tx_outs = vec![
    TxOut {
      tx_id: bob_tx_id,
      index: 0,
      amount: 3,
      public_key: bob_public_key,
    },
    TxOut {
      tx_id: bob_tx_id,
      index: 1,
      amount: 7,
      public_key: alice_public_key,
    },
  ];

  //use the tx_id from alice's transaction
  let bob_tx_ins = vec![TxIn {
    tx_id: tx_id,
    index: 0,
    signature: None,
  }];

  let mut bob_to_alice = Tx {
    id: bob_tx_id,
    tx_ins: bob_tx_ins,
    tx_outs: bob_tx_outs,
  };

  bob_to_alice.sign_input(0, bob_secret_key);

  bank.handle_tx(bob_to_alice);

  assert_eq!(997, bank.fetch_balance(&alice_public_key));
  assert_eq!(3, bank.fetch_balance(&bob_public_key));

  println!(
    "Alice has {} bankcoin, Bob has {} bankcoin. Good job.",
    bank.fetch_balance(&alice_public_key),
    bank.fetch_balance(&bob_public_key)
  );
}
