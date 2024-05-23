#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use blake2::Digest;
use hex::ToHex;
// You can find the hashing algorithms in the exports from sp_core. In order to easily see what is
// available from sp_core, it might be helpful to look at the rust docs:
// https://paritytech.github.io/substrate/master/sp_core/index.html
use sp_core::*;

/// For simplicity in this exercise, we are only working with 128-bit hashes.
const HASH_SIZE: usize = 16;

/// Use the blake2 hashing algorithm to calculate the 128-bit hash of some input data
pub fn hash_with_blake(data: &[u8]) -> [u8; HASH_SIZE] {
    blake2_128(data)
}

/// Use the twox hashing algorithm to calculate the 128-bit hash of some input data
pub fn hash_with_twox(data: &[u8]) -> [u8; HASH_SIZE] {
    twox_128(data)
}

#[derive(Clone, PartialEq, Eq)]
pub enum HashAlgo {
    TwoX,
    Blake2,
}

/// Use the hashing algorithm variant specified in the argument to hash the data
pub fn hash_with(data: &[u8], algorithm: HashAlgo) -> [u8; HASH_SIZE] {
    match algorithm {
        HashAlgo::Blake2 => hash_with_blake(data),
        HashAlgo::TwoX => hash_with_twox(data),
    }
}

/// Return true iff data is the preimage of hash under the specified algorithm
pub fn is_hash_preimage(hash: [u8; HASH_SIZE], data: &[u8], algorithm: HashAlgo) -> bool {
    hash_with(data, algorithm) == hash
}

/// Add an integrity check to some data by using the blake2 hashing algorithm.
///
/// Hashes can also be used to check data integrity! We will implement a version of this using the
/// blake2 hashing algorithm. To append an integrity code to the end of some input, hash the data,
/// and append the 128-bit hash to the data. The result will look like `data | hash(data)`, using |
/// for concatenation.
pub fn add_integrity_check(data: &[u8]) -> Vec<u8> {
    let hash_result = hash_with_blake(data);
    let mut result = Vec::new();
    result.extend_from_slice(data);
    result.extend_from_slice(&hash_result);
    result
}

/// Verify the integrity of some data via the checksum, and return the original data
///
/// In order to verify that the data is valid, we separate it out into the received hash and the
/// original data. Then, we hash the original data and compare it to the received hash. If it is
/// the same, we return the original data. Otherwise, we return an error.
///
/// Note that when receiving data that has an integrity check, it is important that we know
/// _exactly_ how the integrity check was generated. Most of the time, the integrity checks are
/// not able to be self-describing, so the verification end needs to know how to use the
/// integrity check.
pub fn verify_data_integrity(data: Vec<u8>) -> Result<Vec<u8>, ()> {
    if data.len() < 16 {
        return Err(());
    }

    let (original_data, hashed_data) = data.split_at(data.len() - 16);

    let hash_result = hash_with_blake(original_data);
    if hash_result == hashed_data {
        return Ok(original_data.to_vec());
    }
    Err(())
}

use rand::{rngs::SmallRng, seq::IteratorRandom, Rng, SeedableRng};
use std::{cell::RefCell, collections::HashMap};
use strum::{EnumIter, IntoEnumIterator};
type HashValue = [u8; HASH_SIZE];

/// Now that we are comfortable using hashes, let's implement a classic commit-reveal scheme using a
/// public message board. This message board implements some functionality to allow people to communicate.
/// It allows people to commit to a message, and then later reveal that message. It also lets people
/// look up a commitment to see if the message has been revealed or not.
///
/// This message board will use the 128-bit Blake2 hashing algorithm.
#[derive(Debug)]
pub struct PublicMessageBoard {
    /// The commitals to this public message board. A 'None' value represents a commitment that has
    /// not been revealed. A 'Some' value will contain the revealed value corresponding to the
    /// commitment.
    commitals: HashMap<HashValue, Option<String>>,
    /// A seeded RNG used to generate randomness for committing
    ///
    /// STUDENTS: DO NOT USE THIS YOURSELF. The provided code already uses it everywhere necessary.
    rng: SmallRng,
}

impl PublicMessageBoard {
    /// Create a new message board
    pub fn new(rng_seed: u64) -> Self {
        PublicMessageBoard {
            commitals: HashMap::new(),
            rng: SmallRng::seed_from_u64(rng_seed),
        }
    }

    /// Post a commitment to the public message board, returning the message with added randomness
    /// and the commitment to share. If the commitment already exists, this does not modify the
    /// board, but returns the same values.
    ///
    /// The input messages should have some randomness appended to them so that an attacker cannot
    /// guess the messages to crack the hash. For compatibility with tests, do not use the message
    /// board's RNG other than the provided code below.
    ///
    /// Note that in reality, the commitment would be calculated offline, and only the commitment
    /// posted to the message board. However, in this example, we pretend that this is a "frontend"
    /// to the message board that handles that for you.
    pub fn post_commitment(&mut self, msg: String) -> (String, HashValue) {
        let randomness: [u8; 4] = self.rng.gen();
        let randomness_string = hex::encode(randomness);
        let appended_message = format!("{}{}", msg, randomness_string);
        let commitment = hash_with_blake(appended_message.as_bytes());
        let existing_commit = self.commitals.contains_key(&commitment);
        if !existing_commit {
            self.commitals.insert(commitment, None);
        }
        (appended_message, commitment)
    }

    /// Post a reveal for an existing commitment. The input should be the message with randomness added.
    ///
    /// Returns Ok(commitment) if the reveal was successful, or an error if the commitment wasn't
    /// found or has already been revealed.
    pub fn post_reveal(&mut self, committed_msg: String) -> Result<HashValue, ()> {
        let commitment = hash_with_blake(committed_msg.as_bytes());
        let commit_exists = self.commitals.contains_key(&commitment);
        if commit_exists {
            let val = self.commitals.get_mut(&commitment).unwrap();
            match val {
                Some(value) => {
                    return Err(());
                }
                None => {
                    *val = Some(committed_msg);
                }
            }
            return Ok(commitment);
        }
        return Err(());
    }

    /// Check a certain commitment. Errors if the commitment doesn't exist, and otherwise returns
    /// None if the commitment has not been revealed, or the value if it has been revealed.
    pub fn check_commitment(&self, commitment: HashValue) -> Result<Option<String>, ()> {
        let commit_exists = self.commitals.contains_key(&commitment.clone());
        if !commit_exists {
            return Err(());
        }

        let val = self.commitals.get(&commitment.clone()).unwrap();
        return Ok(val.clone());
    }

    /// Helper method to convert from a reveal to the corresponding commitment.
    pub fn reveal_to_commit(reveal: &str) -> HashValue {
        hash_with_blake(reveal.as_bytes())
    }
}
