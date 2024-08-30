use lazy_static;
use phf::phf_map;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};

// Binary Dictionary for Parsing GCTO File
// a, t = 00
// c, g = 01
// space = 10
pub static PARSER_DICTIONARY: phf::Map<char, u8> = phf_map! {
    'a' => 0o0, // 'A' Nucleotide
    'A' => 0o0, // 'A' Nucleotide
    't' => 0o1, // 'T' Nucleotide
    'T' => 0o1, // 'T' Nucleotide
    'c' => 0o2, // 'C' Nucleotide
    'C' => 0o2, // 'C' Nucleotide
    'g' => 0o3, // 'G' Nucleotide
    'G' => 0o3, // 'G' Nucleotide
    ':' => 0o4, // Mark End of Nucleotide Sequence
    ' ' => 0o5, // Mark End of Data Entry
    'n' => 0o6, // Unknown Nucleic Acid Residue
    'N' => 0o6, // Unknown Nucleic Acid Residue
    'x' => 0o7, // Unknown Amino Acid Residue
    'X' => 0o7  // Unknown Amino Acid Residue
};

// Dictionary for Complementary Nucleotide Base Pairing
pub static INVERSION_DICTIONARY: phf::Map<char, u8> = phf_map! {
    'a' => 0o1, // 'A' Nucleotide Base Pair 'T'
    'A' => 0o1, // 'A' Nucleotide Base Pair 'T'
    't' => 0o0, // 'T' Nucleotide Base Pair 'A'
    'T' => 0o0, // 'T' Nucleotide Base Pair 'A'
    'c' => 0o3, // 'C' Nucleotide Base Pair 'G'
    'C' => 0o3, // 'C' Nucleotide Base Pair 'G'
    'g' => 0o2, // 'G' Nucleotide Base Pair 'C'
    'G' => 0o2  // 'G' Nucleotide Base Pair 'C'
};

// Reverse Lookup Table for PARSER_DICTIONARY
lazy_static::lazy_static! {
    static ref REV_PARSER_DICTIONARY: HashMap<u8, char> = {
        let mut rev_map = HashMap::new();
        for (k, v) in PARSER_DICTIONARY.into_iter() {
            rev_map.insert(*v, *k);
        }
        rev_map
    };
}

// Reverse Lookup Table for INVERSION_DICTIONARY
lazy_static::lazy_static! {
    static ref REV_INVERSION_DICTIONARY: HashMap<u8, char> = {
        let mut rev_map = HashMap::new();
        for (k, v) in INVERSION_DICTIONARY.into_iter() {
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

fn u8_to_u32(data: Vec<u8>) -> u32 {
    let mut padded_data: Vec<u8> = Vec::new();

    for _ in 0..(4 - data.len()) {
        padded_data.push(0);
    }
    padded_data.append(&mut data.clone());

    u32::from_be_bytes(padded_data.try_into().unwrap())
}

pub fn generate_gcto(infile_path: String, outfile_name: String) {
    let mut output: Vec<u8> = Vec::new();
    let mut sequence: String = String::new();

    let mut file = File::open(infile_path).unwrap();
    file.read_to_string(&mut sequence).unwrap();

    for char in sequence.chars() {
        if !PARSER_DICTIONARY.contains_key(&char) {
            panic!("PARSER HAS NO DEFINITION FOR {}", char)
        }
        output.push(PARSER_DICTIONARY[&char]);
    }

    File::create(outfile_name + ".gcto").unwrap().write(&output);
}

pub fn manual_create_gcto(data: String, outfile_name: String) {
    let mut output: Vec<u8> = Vec::new();

    for char in data.chars() {
        if !PARSER_DICTIONARY.contains_key(&char) {
            panic!("PARSER HAS NO DEFINITION FOR {}", char)
        }
        output.push(PARSER_DICTIONARY[&char]);
    }

    File::create(outfile_name + ".gcto").unwrap().write(&output);
}

pub fn generate_gcto_frequency_map(
    data: HashMap<Vec<u8>, u32>,
    outfile_name: String,
    override_mapping: Option<bool>,
) {
    let mut outVec: Vec<u8> = Vec::new();
    for pair in data {
        let mut newVec: Vec<u8> = pair.0.clone();
        newVec.push(PARSER_DICTIONARY[&':']);
        newVec.append(&mut u32_to_u8(pair.1));
        newVec.push(PARSER_DICTIONARY[&' ']);
        outVec.append(&mut newVec);
    }

    let out_file = File::create(outfile_name.to_owned().replace('>', "").to_string() + ".gcto");
    out_file.unwrap().write(&outVec);
}

pub fn load_gcto_table(filePath: &String) -> Vec<(String, u32)> {
    let mut output_table: Vec<(String, u32)> = Vec::new();

    let mut raw_data = fs::read(filePath).ok().unwrap();
    raw_data.push(0o0);

    let mut sequence_builder: String = "".to_string();
    let mut is_building_sequence: bool = true;

    let mut count_builder: Vec<u8> = Vec::new();

    // Write a general for loop to read the GCTO file

    for n in 0..raw_data.len() - 1 {
        //if !REV_PARSER_DICTIONARY.contains_key(&raw_data[n]) {
        //    println!("PARSER HAS NO DEFINITION FOR {}", raw_data[n]);
        //}
        //if !REV_PARSER_DICTIONARY.contains_key(&raw_data[n+1]) {
        //    println!("PARSER HAS NO DEFINITION FOR NEXT BYTE {}", raw_data[n+1]);
        //}

        //print!("{} ", raw_data[n]);
        if REV_PARSER_DICTIONARY.contains_key(&raw_data[n]) {
            if REV_PARSER_DICTIONARY[&raw_data[n]] == ':' {
                is_building_sequence = false;
                continue;
            } else if REV_PARSER_DICTIONARY[&raw_data[n]] == ' '
                && n < raw_data.len()
                && REV_PARSER_DICTIONARY.get(&raw_data[n + 1]).unwrap_or(&' ') != &' '
            {
                output_table.push((sequence_builder, u8_to_u32(count_builder)));

                sequence_builder = "".to_string();
                count_builder = Vec::new();
                is_building_sequence = true;

                continue;
            }
        }

        if is_building_sequence {
            sequence_builder.push(REV_PARSER_DICTIONARY[&raw_data[n]]);
        } else {
            count_builder.push(raw_data[n]);
        }
    }

    if output_table.is_empty() {
        panic!("Output Table is Empty, GCTO Unable to be Loaded");
    }

    output_table
}

pub fn string_to_gcto(sequence: String) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    for char in sequence.chars() {
        if !PARSER_DICTIONARY.contains_key(&char) {
            panic!("PARSER HAS NO DEFINITION FOR {}", char)
        }
        output.push(PARSER_DICTIONARY[&char]);
    }

    return output;
}

pub fn gcto_to_string(sequence: Vec<u8>) -> String {
    let mut output: String = String::new();

    for byte in sequence {
        if !REV_PARSER_DICTIONARY.contains_key(&byte) {
            panic!("PARSER HAS NO DEFINITION FOR {}", byte)
        }
        output.push(REV_PARSER_DICTIONARY[&byte]);
    }

    return output;
}
