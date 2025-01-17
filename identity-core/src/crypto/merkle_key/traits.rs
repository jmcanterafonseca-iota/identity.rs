// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::merkle_key::MerkleTag;
use crate::crypto::merkle_tree::DigestExt;

/// A common interface for signature algorithms supported by Merkle Key Signatures.
pub trait MerkleSignature {
  /// A unique tag identifying the signature algorithm.
  const TAG: MerkleTag;
}

/// A common interface for digest algorithms supported by Merkle Key Signatures.
pub trait MerkleDigest: DigestExt + 'static {
  /// A unique tag identifying the digest algorithm.
  const TAG: MerkleTag;
}
