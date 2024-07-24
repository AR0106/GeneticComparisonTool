mod analyzers {
    pub mod targeting;
}
mod gcto_file;
mod charts;

use analyzers::targeting;
use gcto_file::{generate_gcto, manual_create_gcto};
use regex::Regex;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{Read, Write},
    thread,
};

const MAX_LENGTH: u32 = 9;
const MIN_LENGTH: u32 = 6;
const RETAINMENT_THRESHOLD: u32 = 10;

fn main() {
    // Collects Arguments Passed Into Program
    let args: Vec<String> = env::args().collect();

    // Open FNA File
    // Index 1 Contains First Past Argument (Should Be FNA File Path)

    if args[1] == "generate_frequency" {
        load_fasta_into_gcto(&args);
    } else if args[1] == "load_frequency" {
        let table = gcto_file::load_gcto_table(&args[2]);

        for entry in table {
            println!("{:?}", entry);
        }
    } else if args[1] == "convert" {
        generate_gcto(args[2].to_string(), args[3].to_string());
    } else if args[1] == "find_pam" {
        let mut genome: Vec<u8> = Vec::new();

        File::open(&args[2])
            .unwrap()
            .read_to_end(&mut genome)
            .unwrap();

        let pam_sites: Vec<u64> = targeting::find_pam_sites(genome);

        for site in pam_sites {
            println!("PAM Site Found at Index {}", site);
        }
    } else if args[1] == "find_chi" {
        let mut genome: Vec<u8> = Vec::new();
        File::open(&args[2])
            .unwrap()
            .read_to_end(&mut genome)
            .unwrap();
        let chi_sites: Vec<u64> = targeting::find_chi_sites(genome);
        for site in chi_sites {
            println!("Chi Site Found at Index {}", site);
        }
    } else if args[1] == "create_genome" {
        let genome_name = &args[2];
        let genome = &args[3];

        gcto_file::manual_create_gcto(genome.to_string(), genome_name.to_string());
    } else if args[1] == "generate_single_frequency" {
        let outfile = &args[2];
        let genome = &args[3];
        let map = analyze_sequence_as_string(genome.to_string());

        for (key, value) in &map {
            println!("{}: {}", key, value);
        }

        manual_create_gcto(genome.to_string(), outfile.to_string());

        let include_json_output: bool = args.contains(&"--json".to_string());

        if include_json_output {
            let decoded_file = File::create("single-frequency.json");
            decoded_file
                .unwrap()
                .write(serde_json::to_string(&map).unwrap().as_bytes());
        }
    } else if args[1] == "generate_chart" {
        let genome_data = gcto_file::load_gcto_table(&args[2]);

        let mut map: HashMap<String, u32> = HashMap::new();
        genome_data.iter().for_each(|entry| {
            map.insert(entry.0.clone(), entry.1);
        });
        
        charts::generate_chart(map);
    } else {
        println!("Invalid Command");
    }
}

fn load_fasta_into_gcto(args: &Vec<String>) {
    let fasta = File::open(&args[2]);
    let mut genomic_data = String::new();

    // Read File into genomic_data String
    fasta.unwrap().read_to_string(&mut genomic_data);

    // Parse FNA File into Headers and Their Respective Data
    // This Creates 2 Lists with Matching Indices
    let pattern = Regex::new(r">(.*)").unwrap();
    let chromosome_rna: Vec<&str> = pattern.split(genomic_data.as_str()).collect();
    let chromosome_list: Vec<&str> = pattern
        .find_iter(genomic_data.as_str())
        .map(|mat| &genomic_data[mat.start()..mat.end()])
        .collect();

    let mut progress: u32 = 0;

    let include_json_output: bool = args.contains(&"--json".to_string());

    //let mut chromosome_list: Vec<&str> = Vec::new();
    //chromosome_list.push("sampleGenome");

    //let mut chromosome_rna: Vec<&str> = Vec::new();
    //chromosome_rna.push("atcaatgatcaacgtaagcttctaagcatgatcaaggtgctcacacagtttatccacaacctgagtggatgacatcaagataggtcgttgtatctccttcctctcgtactctcatgaccacggaaagatgatcaagagaggatgatttcttggccatatcgcaatgaatacttgtgacttgtgcttccaattgacatcttcagcgccatattgcgctggccaaggtgacggagcgggattacgaaagcatgatcatggctgttgttctgtttatcttgttttgactgagacttgttaggatagacggtttttcatcactgactagccaaagccttactctgcctgacatcgaccgtaaattgataatgaatttacatgcttccgcgacgatttacctcttgatcatcgatccgattgaagatcttcaattgttaattctcttgcctcgactcatagccatgatgagctcttgatcatgtttccttaaccctctattttttacggaagaatgatcaagctgctgctcttgatcatcgtttc");

    // Look For Matches in Chromosome
    // Uses a Sliding Window that Searches for Matches of One Size, Then Goes Back and Increases
    // the Size by 1 Until the MAX_LENGTH Consntant Is Reached
    thread::scope(|scope| {
        for chromosome in 0..chromosome_list.len() {
            // Create Copies of 'chromosome_rna' and 'chromosome_list' to be Used in Closure
            let chromosome_rna_clone = chromosome_rna.clone(); // Clone chromosome_rna
            let list_copy_clone = chromosome_list.clone(); // Clone chromosome_list
            scope.spawn(move || {
                let rna_copy = chromosome_rna_clone; // Use the cloned value inside the closure
                let list_copy = list_copy_clone; // Use the cloned value inside the closure

                let genome = &rna_copy[chromosome].to_lowercase();

                let map = analyze_sequence_as_string(genome.to_string());

                gcto_file::generate_gcto_frequency_map(
                    map.clone(),
                    list_copy[chromosome].to_string(),
                    None,
                );

                // Allow the User to Generate ".json" Versions of the ".gcto" files
                if include_json_output {
                    let decoded_file = File::create(
                        list_copy[chromosome]
                            .to_owned()
                            .replace('>', "")
                            .to_string()
                            + ".json",
                    );
                    decoded_file
                        .unwrap()
                        .write(serde_json::to_string(&map).unwrap().as_bytes());
                }
            });

            // TODO: FIX PROGESS BAR
            progress += 1;
            println!("{}/{}", progress, chromosome_list.len());
        }
    });
}

// Iterate Through Each Sequence MAX_LENGTH Times and put it in a HashMap
// @param sequence: String
// @return HashMap<String, u32>
fn analyze_sequence_as_string(sequence: String) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();

    for i in MIN_LENGTH..MAX_LENGTH {
        let mut left: usize = 0;
        let mut right: usize = i.try_into().unwrap();

        for _j in 0..sequence.len() {
            let mut _genome = &sequence[left..right];
            //println!("{}", _genome);
            let _genome = _genome.replace('\n', "");
            map.entry(_genome).and_modify(|val| *val += 1).or_insert(1);

            if left < sequence.len() && right < sequence.len() {
                left += 1;
                right += 1;
            }
        }
    }

    map.retain(|_, v| *v > RETAINMENT_THRESHOLD);

    return map;
}

fn analyze_sequence(sequence: Vec<u8>) -> HashMap<Vec<u8>, u32> {
    let mut map: HashMap<Vec<u8>, u32> = HashMap::new();

    for i in MIN_LENGTH..MAX_LENGTH {
        let mut left: usize = 0;
        let mut right: usize = i.try_into().unwrap();

        for _ in 00..sequence.len() {
            let mut genome = &sequence[left..right];
            map.entry(genome.to_vec()).and_modify(|val| *val += 1).or_insert(1);

            if left < sequence.len() && right < sequence.len() {
                left += 1;
                right += 1;
            }
        }
    }

    map.retain(|_, v| *v > RETAINMENT_THRESHOLD);

    return map;
}