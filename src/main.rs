mod gcto_file;

use regex::Regex;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{Read, Write},
    thread,
};

const MAX_LENGTH: u32 = 9;

fn main() {
    // Collects Arguments Passed Into Program
    let args: Vec<String> = env::args().collect();

    // Open FNA File
    // Index 1 Contains First Past Argument (Should Be FNA File Path)
    
    if args[1] == "generate" {
        load_fasta_into_gcto(&args);
    }
    else if args[1] == "load" {
        let table = gcto_file::load_gcto(&args[2]);

        for entry in table {
            println!("{:?}", entry);
        }
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

                let map = analyze_sequence(genome.to_string());

                gcto_file::generate_gcto(map.clone(), list_copy[chromosome].to_string(), None);

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
fn analyze_sequence(sequence: String) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();

    for i in 2..MAX_LENGTH {
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

    map.retain(|_, v| *v > 1);

    return map;
}
