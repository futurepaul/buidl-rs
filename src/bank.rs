use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::collections::HashMap;
use uuid::Uuid;

use crate::tx::{Tx, TxIn, TxOut};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Outpoint {
  pub tx_id: Uuid,
  pub index: usize,
}

#[derive(Debug)]
pub struct Bank {
  pub utxo: HashMap<Outpoint, TxOut>,
  pub secp: Secp256k1<secp256k1::All>,
}

impl Bank {
  fn update_utxo(&mut self, tx: &Tx) {
    for tx_in in tx.tx_ins.iter() {
      self.utxo.remove(&tx_in.clone().outpoint());
    }
    for tx_out in tx.tx_outs.iter() {
      self.utxo.insert(tx_out.clone().outpoint(), tx_out.clone());
    }
  }
  pub fn issue(&mut self, amount: u64, public_key: PublicKey) -> Tx {
    let id = Uuid::new_v4();
    let tx_ins: Vec<TxIn> = Vec::new();
    let tx_outs = vec![TxOut {
      tx_id: id,
      index: 0,
      amount: amount,
      public_key: public_key,
    }];

    let tx = Tx {
      id: id,
      tx_ins: tx_ins,
      tx_outs: tx_outs,
    };

    self.update_utxo(&tx);

    tx
  }

  fn validate_tx(&self, tx: Tx) {
    let mut in_sum = 0;
    let mut out_sum = 0;

    for tx_in in tx.tx_ins.iter() {
      assert!(
        self.utxo.contains_key(&tx_in.clone().outpoint()),
        "TxIn not in UTXO HashMap!"
      );

      let tx_out = self.utxo.get(&tx_in.clone().outpoint()).unwrap();

      let verification = self.secp.verify(
        &tx_in.spend_message(),
        &tx_in.signature.unwrap(),
        &tx_out.public_key,
      );

      match verification {
        Ok(_) => println!("Verified tx with UUID: {}", tx_in.tx_id),
        Err(e) => panic!("Verification failed because: {}", e),
      }

      let amount = tx_out.amount;
      in_sum += amount;
    }

    for tx_out in tx.tx_outs.iter() {
      out_sum += tx_out.amount;
    }

    assert!(in_sum == out_sum);
  }

  pub fn handle_tx(&mut self, tx: Tx) {
    self.validate_tx(tx.clone());
    self.update_utxo(&tx);
  }

  fn fetch_utxo(&self, public_key: &PublicKey) -> Vec<TxOut> {
    let mut tx_outs = Vec::new();
    for tx_out in self
      .utxo
      .values()
      .filter(|&tx_out| tx_out.public_key == *public_key)
    {
      tx_outs.push(tx_out.clone());
    }
    tx_outs
  }

  pub fn fetch_balance(&self, public_key: &PublicKey) -> u64 {
    let unspents = self.fetch_utxo(public_key);

    let mut balance = 0;

    for unspent in unspents.iter() {
      balance += unspent.amount;
    }

    balance
  }
}
