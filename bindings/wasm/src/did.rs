// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::iota::DID as IotaDID;
use wasm_bindgen::prelude::*;

use crate::crypto::KeyPair;
use crate::utils::err;

/// @typicalname did
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct DID(pub(crate) IotaDID);

#[wasm_bindgen]
impl DID {
  /// Creates a new `DID` from a `KeyPair` object.
  #[wasm_bindgen(constructor)]
  pub fn new(key: &KeyPair, network: Option<String>, shard: Option<String>) -> Result<DID, JsValue> {
    let public: &[u8] = key.0.public().as_ref();
    let network: Option<&str> = network.as_deref();
    let shard: Option<&str> = shard.as_deref();

    IotaDID::from_components(public, network, shard).map_err(err).map(Self)
  }

  /// Creates a new `DID` from a base58-encoded public key.
  #[wasm_bindgen(js_name = fromBase58)]
  pub fn from_base58(key: &str, network: Option<String>, shard: Option<String>) -> Result<DID, JsValue> {
    let public: Vec<u8> = decode_b58(key).map_err(err)?;
    let network: Option<&str> = network.as_deref();
    let shard: Option<&str> = shard.as_deref();

    IotaDID::from_components(&public, network, shard).map_err(err).map(Self)
  }

  /// Parses a `DID` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<DID, JsValue> {
    IotaDID::parse(input).map_err(err).map(Self)
  }

  /// Returns the IOTA tangle network of the `DID`.
  #[wasm_bindgen(getter)]
  pub fn network(&self) -> String {
    self.0.network().into()
  }

  /// Returns the IOTA tangle shard of the `DID` (if any).
  #[wasm_bindgen(getter)]
  pub fn shard(&self) -> Option<String> {
    self.0.shard().map(Into::into)
  }

  /// Returns the unique tag of the `DID`.
  #[wasm_bindgen(getter)]
  pub fn tag(&self) -> String {
    self.0.tag().into()
  }

  /// Returns the `DID` object as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}
