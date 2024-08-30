use lazy_static;
use std::collections::HashMap;

#[path = "../gcto_file.rs"]
mod gcto_file;

lazy_static::lazy_static! {
    static ref COMPLEMENT_DICTIONARY: HashMap<u8, u8> = {
       let tmp_map =HashMap::from([
            (0o0, 0o1), // 'A' Nucleotide Base Pair 'T'
            (0o1, 0o0), // 'T' Nucleotide Base Pair 'A'
            (0o2, 0o3), // 'G' Nucleotide Base Pair 'C'
            (0o3, 0o2), // 'C' Nucleotide Base Pair 'G'
        ]);

        tmp_map
    };
}

pub fn complement_sequence(sequence: &Vec<u8>) -> Vec<u8> {
    let mut complemented_sequence: Vec<u8> = Vec::new();

    for nucleotide in sequence {
        complemented_sequence.push(COMPLEMENT_DICTIONARY.get(&nucleotide).unwrap().to_owned())
    }

    complemented_sequence
}

pub fn reverse_sequence(mut sequence: Vec<u8>) -> Vec<u8> {
    sequence.reverse();

    sequence
}

pub fn reverse_complement(sequence: Vec<u8>) -> Vec<u8> {
    let complement = complement_sequence(&sequence);

    reverse_sequence(complement)
}
