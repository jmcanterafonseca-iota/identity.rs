// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::did::DID;

macro_rules! get {
  (@network $this:expr) => {
    &$this.0[..get!(@head $this)]
  };
  (@shard $this:expr) => {
    &$this.0[&get!(@head $this) + 1..get!(@tail $this)]
  };
  (@tag $this:expr) => {
    &$this.0[get!(@tail $this) + 1..]
  };
  (@head $this:expr) => {
    // unwrap is fine - we only operate on valid DIDs
    $this.0.find(':').unwrap()
  };
  (@tail $this:expr) => {
    // unwrap is fine - we only operate on valid DIDs
    $this.0.rfind(':').unwrap()
  };
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segments<'id>(pub(crate) &'id str);

impl<'id> Segments<'id> {
  pub fn is_default_network(&self) -> bool {
    match self.count() {
      1 => true,
      2 | 3 => get!(@network self) == DID::DEFAULT_NETWORK,
      _ => unreachable!("Segments::is_default_network called for invalid IOTA DID"),
    }
  }

  pub fn network(&self) -> &'id str {
    match self.count() {
      1 => DID::DEFAULT_NETWORK,
      2 | 3 => get!(@network self),
      _ => unreachable!("Segments::network called for invalid IOTA DID"),
    }
  }

  pub fn shard(&self) -> Option<&'id str> {
    match self.count() {
      1 | 2 => None,
      3 => Some(get!(@shard self)),
      _ => unreachable!("Segments::shard called for invalid IOTA DID"),
    }
  }

  pub fn tag(&self) -> &'id str {
    match self.count() {
      1 => self.0,
      2 | 3 => get!(@tag self),
      _ => unreachable!("Segments::tag called for invalid IOTA DID"),
    }
  }

  pub fn count(&self) -> usize {
    self.0.split(':').count()
  }
}
