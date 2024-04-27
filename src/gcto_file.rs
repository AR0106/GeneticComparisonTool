use lazy_static;
use phf::{phf_map, PhfHash};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;

// Binary Dictionary for Parsing GCTO File
// a, t = 00
// c, g = 01
// space = 10
static PARSER_DICTIONARY: phf::Map<char, u8> = phf_map! {
    'a' => 0o00,
    'A' => 0o00,
    't' => 0o01,
    'T' => 0o01,
    'c' => 0o02,
    'C' => 0o02,
    'g' => 0o03,
    'G' => 0o03,
    ':' => 0o04,
    ' ' => 0o05
};

lazy_static::lazy_static! {
    static ref REV_PARSER_DICTIONARY: HashMap<u8, char> = {
        let mut rev_map = HashMap::new();
        for (k, v) in PARSER_DICTIONARY.into_iter() {
            rev_map.insert(*v, *k);
        }
        rev_map
    };
}

fn u32_to_u8(data: u32) -> Vec<u8> {
    let bytes = data.to_be_bytes();
    let mut reducedBytes: Vec<u8> = Vec::new();

    for &byte in bytes.iter() {
        if byte != 0 {
            reducedBytes.push(byte);
        }
    }

    reducedBytes
}

pub fn generate_gcto(data: HashMap<String, u32>, outfile_name: String) {
    let mut output_table: Vec<(Vec<u8>, u32)> = Vec::new();

    for (key, value) in data {
        let sequence: Vec<u8> = key.chars().map(|n| PARSER_DICTIONARY[&n]).collect();
        output_table.push((sequence, value));
    }

    let mut outVec: Vec<u8> = Vec::new();
    for pair in &output_table {
        let mut newVec: Vec<u8> = pair.0.clone();
        newVec.push(PARSER_DICTIONARY[&':']);
        newVec.append(&mut u32_to_u8(pair.1));
        newVec.push(PARSER_DICTIONARY[&' ']);
        outVec.append(&mut newVec);
    }

    let out_file = File::create(outfile_name.to_owned().replace('>', "").to_string() + ".gcto");
    out_file.unwrap().write(&outVec);
}

/*
pub fn load_gcto(filePath: String) -> Vec<(String, u32)> {
    let mut output_table: Vec<(String, u32)> = Vec::new();

    let raw_data = fs::read(filePath);
}*/
