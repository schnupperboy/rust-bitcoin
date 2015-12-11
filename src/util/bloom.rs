//! # Bloom Filters
//!
//! A Bloom filter is a bit-field in which bits are set based on feeding the data element to a set of different hash functions.
//! The number of hash functions used is a parameter of the filter. In Bitcoin we use version 3 of the 32-bit Murmur hash function.

use std::cmp::min;
use std::fmt;
use rand;
use murmur3::{seeded, unseeded};

const MAX_BLOOM_FILTER_SIZE: u16 = 520; // bytes
const MAX_HASH_FUNCS: u8 = 50;
const LN2_SQUARED: f32 = 0.4804530139182014246671025263266649717305529515945455;
const LN2: f32 = 0.6931471805599453094172321214581765680755001343602552;
const SEED_COEF: u32 = 0xfba4c795;
const BIT_MASK: [u8; 8] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];

#[derive(Debug)]
/// The type for a bloom filter
pub struct BloomFilter {
	data: Vec<u8>,
	n_hash_funcs: u32,
	n_tweak: u32,
	n_flags: u8
}

impl fmt::Binary for BloomFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.data.len() {
	    	match write!(f, "{:01$b} ", self.data[i], 8) {
	    		Ok(_) => (),
	    		Err(e) => return Err(e)
	    	}
	    }

	    Ok(())
    }
}

impl BloomFilter {
	fn new(n_elements: u32, fp_rate: f32, n_flags: u8) -> BloomFilter {
        let data_size = -1.0  / LN2_SQUARED * (n_elements as f32) * fp_rate.ln() / 8.0;
		let data_size = min(data_size as usize, MAX_BLOOM_FILTER_SIZE as usize);

		let n_hash_funcs = data_size as f32 * 8.0 / (n_elements as f32) * LN2;
		let n_hash_funcs = min(n_hash_funcs as u32, MAX_HASH_FUNCS as u32);

		BloomFilter {
			data: vec!(0u8; data_size),
			n_hash_funcs: n_hash_funcs,
			n_tweak: rand::random(),
			n_flags: n_flags
		}
	}

	fn hash(&self, n_hash_func: u32, data: &[u8]) -> u32 {
		let seed = n_hash_func.wrapping_mul(SEED_COEF).wrapping_add(self.n_tweak);
		seeded(data, seed) % (self.data.len() as u32 * 8)
	}

	fn insert(& mut self, data: &[u8]) {
		for i_hash_func in 0..self.n_hash_funcs {
		    let i_data = self.hash(i_hash_func, &data);
		    let offset= (i_data >> 3) as usize;
		    self.data[offset] |= BIT_MASK[(7 & i_data) as usize]
		}
	}

	fn contains(&self, data: &[u8]) -> bool {
		for i_hash_func in 0..self.n_hash_funcs {
		    let i_data = self.hash(i_hash_func, &data);
		    let offset= (i_data >> 3) as usize;

		 	if 0 == (self.data[offset] & BIT_MASK[(7 & i_data) as usize]) {
		 		return false;
		 	}
		}

		true
	}
}

#[test]
fn create() {
	let mut bl = BloomFilter::new(1, 0.0001, 0);

	assert_eq!(bl.data.len(), 2);
	assert_eq!(bl.n_hash_funcs, 11);
}

#[test]
fn create_insert() {
	let mut bl = BloomFilter::new(3, 0.01, 0);
	bl.n_tweak = 0;

	bl.insert("99108ad8ed9bb6274d3980bab5a85c048f0950c8".as_bytes());
	bl.insert("b5a2c786d9ef4658287ced5914b37a1b4aa32eee".as_bytes());
	bl.insert("b9300670b4c5366e95b2699e8b18bc75e5f729c5".as_bytes());
	assert!(bl.contains("99108ad8ed9bb6274d3980bab5a85c048f0950c8".as_bytes()));
	assert!(!bl.contains("19108ad8ed9bb6274d3980bab5a85c048f0950c8".as_bytes()));
	assert!(bl.contains("b5a2c786d9ef4658287ced5914b37a1b4aa32eee".as_bytes()));
	assert!(bl.contains("b9300670b4c5366e95b2699e8b18bc75e5f729c5".as_bytes()));
}

#[test]
fn create_insert_with_tweak() {
	assert_eq!(0b00000000, 0b00000000 & 0b10000000);
	let mut bl = BloomFilter::new(3, 0.01, 0);
	bl.n_tweak = 2147483649;

	bl.insert("99108ad8ed9bb6274d3980bab5a85c048f0950c8".as_bytes());
	bl.insert("b5a2c786d9ef4658287ced5914b37a1b4aa32eee".as_bytes());
	bl.insert("b9300670b4c5366e95b2699e8b18bc75e5f729c5".as_bytes());
	assert!(bl.contains("99108ad8ed9bb6274d3980bab5a85c048f0950c8".as_bytes()));
	assert!(!bl.contains("19108ad8ed9bb6274d3980bab5a85c048f0950c8".as_bytes()));
	assert!(bl.contains("b5a2c786d9ef4658287ced5914b37a1b4aa32eee".as_bytes()));
	assert!(bl.contains("b9300670b4c5366e95b2699e8b18bc75e5f729c5".as_bytes()));
}
