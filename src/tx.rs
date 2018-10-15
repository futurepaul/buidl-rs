use secp256k1::{PublicKey, Secp256k1, SecretKey, Signature};
use uuid::Uuid;

use crate::bank::Outpoint;
use crate::util::hash_256_from_string;

#[derive(Debug, Clone)]
pub struct Tx {
  pub id: Uuid,
  pub tx_ins: Vec<TxIn>,
  pub tx_outs: Vec<TxOut>,
}

impl Tx {
  pub fn sign_input(&mut self, index: usize, private_key: SecretKey) {
    let message = self.tx_ins[index].spend_message();
    let secp = Secp256k1::new();
    let signature = secp.sign(&message, &private_key);

    self.tx_ins[index].signature = Some(signature);
  }
}

#[derive(Debug, Clone)]
pub struct TxIn {
  pub tx_id: Uuid,
  pub index: usize,
  pub signature: Option<Signature>,
}

impl TxIn {
  pub fn outpoint(&self) -> Outpoint {
    Outpoint {
      tx_id: self.tx_id,
      index: self.index,
    }
  }

  pub fn spend_message(&self) -> secp256k1::Message {
    let message_string = format!("{}:{}", self.tx_id, self.tx_id);
    let hash = hash_256_from_string(message_string);
    secp256k1::Message::from_slice(&hash).expect("32 bytes")
  }
}

#[derive(Debug, Clone)]
pub struct TxOut {
  pub tx_id: Uuid,
  pub index: usize,
  pub amount: u64,
  pub public_key: PublicKey,
}

impl TxOut {
  pub fn outpoint(self) -> Outpoint {
    Outpoint {
      tx_id: self.tx_id,
      index: self.index,
    }
  }
}
