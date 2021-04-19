// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//!
//! cargo run --example resolve_did

use std::env;
use identity::iota::Client;
use identity::iota::Result;
use identity::did::resolution;
use identity::core::ToJson;

#[smol_potat::main]
async fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    panic!("Please provide a DID");
  }

  // Initialize a `Client` to interact with the IOTA Tangle.
  let client: Client = Client::new()?;

  // Retrieve the published DID Document from the Tangle.
  let future: _ = resolution::resolve(&args[1], Default::default(), &client);
  let did_document: _ = future.await?;

  println!("Resolution: {:#?}", did_document.document.to_json()?);

  Ok(())
}
