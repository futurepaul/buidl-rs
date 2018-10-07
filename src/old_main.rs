extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate png;
extern crate serde_pickle;

use serde::{de, ser};
use std::fs::File;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct PNGCoin {
  transfers: Vec<String>,
}

fn main() {
  let coin_path = "bobs-second.pngcoin";
  let coin = File::open(coin_path).expect("Couldn't open the coin file");

  let deserialized: Vec<String> = serde_pickle::from_reader(&coin).unwrap();

  let something = deserialized;

  println!("{:#?}", something)
}
