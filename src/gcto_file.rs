use phf::{phf_map, PhfHash};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// Binary Dictionary for Parsing GCTO File
// a, t = 00
// c, g = 01
// space = 10
static PARSER_DICTIONARY: phf::Map<char, u8> = phf_map! {
    'a' => 0o00,
    't' => 0o01,
    'c' => 0o02,
    'g' => 0o03,
    ':' => 0o04,
    ' ' => 0o05
};

fn u32_to_u8(data: u32) -> [u8; 4] {
    data.to_be_bytes()
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
        newVec.append(&mut u32_to_u8(pair.1).to_vec());
        newVec.push(PARSER_DICTIONARY[&' ']);
        outVec.append(&mut newVec);
    }

    let out_file = File::create(outfile_name.to_owned().replace('>', "").to_string() + ".gcto");
    out_file.unwrap().write(&outVec);
}
