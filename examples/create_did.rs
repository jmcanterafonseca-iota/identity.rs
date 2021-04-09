// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//!
//! cargo run --example create_did

mod common;

use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::Document;
use identity::iota::Result;
use identity::core::encode_b58;


#[smol_potat::main]
async fn main() -> Result<()> {
  // Initialize a `Client` to interact with the IOTA Tangle.
  let client: Client = Client::new()?;

  // Create a DID Document/KeyPair for the credential issuer.
  let (doc, key_pair): (Document, KeyPair) = common::document(&client).await?;

  println!("Public Key > {}", encode_b58(key_pair.public()));
  println!("Private Key > {}", encode_b58(key_pair.secret()));

  Ok(())
}
