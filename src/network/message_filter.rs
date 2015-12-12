//! # Network filter related messages
//!
//! This module adds support for filtering certain network messages of interest which
//! allows peers to reduce the amount of transaction data they are sent.

use util::hash::Sha256dHash;

/// The `filterload` message
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FilterLoadMessage {
	/// The filter itself is simply a bit field of arbitrary byte-aligned size. The maximum size is 36,000 bytes
	pub filter: Vec<u8>,
	/// The number of hash functions to use in this filter. The maximum value allowed in this field is 50
	pub n_hash_funcs: u32,
	/// A random value to add to the seed value in the hash function used by the bloom filter
	pub n_tweak: u32,
	/// A set of flags that control how matched items are added to the filter
	pub n_flags: u8
}

/// The `filteradd` message
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FilterAddMessage {
	/// The data element to add to the current filter
	pub data: Vec<u8>
}

/// The `merkleblock` message
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MerkleBlockMessage {
	/// Block version information, based upon the software version creating this block
	pub version: u32,
	/// The hash value of the previous block this particular block references
	pub prev_block: Sha256dHash,
	/// The reference to a Merkle tree collection which is a hash of all transactions related to this block
	pub merkle_root: Sha256dHash,
	/// A timestamp recording when this block was created (Limited to 2106!)
	pub timestamp: u32,
	/// The calculated difficulty target being used for this block
	pub bits: u32,
	/// The nonce used to generate this block… to allow variations of the header and compute different hashes
	pub nonce: u32,
	/// Number of transactions in the block (including unmatched ones)
	pub total_transactions: u32,
	/// hashes in depth-first order (including standard varint size prefix)
	pub hashes: Vec<Sha256dHash>,
	/// flag bits, packed per 8 in a byte, least significant bit first (including standard varint size prefix)
	pub flags: Vec<u8>
}

impl FilterLoadMessage {
    /// Construct a new `filterload` message
    pub fn new(filter: Vec<u8>, n_hash_funcs: u32, n_tweak: u32, n_flags: u8) -> FilterLoadMessage {
        FilterLoadMessage {
            filter: filter,
            n_hash_funcs: n_hash_funcs,
            n_tweak: n_tweak,
            n_flags: n_flags
        }
    }
}

impl_consensus_encoding!(FilterLoadMessage, filter, n_hash_funcs, n_tweak, n_flags);

impl FilterAddMessage {
    /// Construct a new `filteradd` message
    pub fn new(data: Vec<u8>) -> FilterAddMessage {
        FilterAddMessage {
            data: data
        }
    }
}

impl_consensus_encoding!(FilterAddMessage, data);

impl MerkleBlockMessage {
    /// Construct a new `merkeblock` message
    pub fn new(version: u32, prev_block: Sha256dHash, merkle_root: Sha256dHash, timestamp: u32, bits: u32, nonce: u32, total_transactions: u32, hashes: Vec<Sha256dHash>, flags: Vec<u8>) -> MerkleBlockMessage {
        MerkleBlockMessage {
            version: version,
            prev_block: prev_block,
            merkle_root: merkle_root,
            timestamp: timestamp,
            bits: bits,
            nonce: nonce,
            total_transactions: total_transactions,
            hashes: hashes,
            flags: flags
        }
    }
}

impl_consensus_encoding!(MerkleBlockMessage, version, prev_block, merkle_root, timestamp, bits, nonce, total_transactions, hashes, flags);
