use regex::Regex;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{Read, Write},
};

fn main() {
    // Collects Arguments Passed Into Program
    let args: Vec<String> = env::args().collect();

    // Open FNA File
    // Index 1 Contains First Past Argument (Should Be FNA File Path)
    let fna = File::open(&args[1]);
    let mut genomic_data = String::new();

    // Read File into genomic_data String
    fna.unwrap().read_to_string(&mut genomic_data);

    // Parse FNA File into Headers and Their Respective Data
    // This Creates 2 Lists with Matching Indices
    let pattern = Regex::new(r">(.*)").unwrap();
    let chromosome_rna: Vec<&str> = pattern.split(genomic_data.as_str()).collect();
    let chromosome_list: Vec<&str> = pattern
        .find_iter(genomic_data.as_str())
        .map(|mat| &genomic_data[mat.start()..mat.end()])
        .collect();

    //let mut chromosome_list: Vec<&str> = Vec::new();
    //chromosome_list.push("sampleGenome");

    //let mut chromosome_rna: Vec<&str> = Vec::new();
    //chromosome_rna.push("atcaatgatcaacgtaagcttctaagcatgatcaaggtgctcacacagtttatccacaacctgagtggatgacatcaagataggtcgttgtatctccttcctctcgtactctcatgaccacggaaagatgatcaagagaggatgatttcttggccatatcgcaatgaatacttgtgacttgtgcttccaattgacatcttcagcgccatattgcgctggccaaggtgacggagcgggattacgaaagcatgatcatggctgttgttctgtttatcttgttttgactgagacttgttaggatagacggtttttcatcactgactagccaaagccttactctgcctgacatcgaccgtaaattgataatgaatttacatgcttccgcgacgatttacctcttgatcatcgatccgattgaagatcttcaattgttaattctcttgcctcgactcatagccatgatgagctcttgatcatgtttccttaaccctctattttttacggaagaatgatcaagctgctgctcttgatcatcgtttc");

    const MAX_LENGTH: u32 = 9;

    // Look For Matches in Chromosome
    // Uses a Sliding Window that Searches for Matches of One Size, Then Goes Back and Increases
    // the Size by 1 Until the MAX_LENGTH Consntant Is Reached
    // TODO: ABSTRACT INTO OWN FUNCTION (POSSIBLY RECURSIVE?)
    for chromosome in 0..chromosome_list.len() {
        let mut map: HashMap<String, i32> = HashMap::new();
        let genome = &chromosome_rna[chromosome].to_lowercase();

        println!("{}/{}", chromosome + 1, chromosome_list.len());

        for i in 2..MAX_LENGTH {
            let mut left: usize = 0;
            let mut right: usize = i.try_into().unwrap();

            for _j in 0..genome.len() {
                let mut _genome = &genome[left..right];
                println!("{}", _genome);
                let _genome = _genome.replace('\n', "");
                map.entry(_genome).and_modify(|val| *val += 1).or_insert(1);

                if left < genome.len() && right < genome.len() {
                    left += 1;
                    right += 1;
                }
            }
        }

        map.retain(|_, v| *v > 1);

        // Create File and Write Matches to Output
        // TODO: ABSTRACT INTO OWN FUNCTION
        let decoded_file = File::create(
            chromosome_list[chromosome]
                .to_owned()
                .replace('>', "")
                .to_string()
                + ".txt",
        );
        decoded_file
            .unwrap()
            .write(serde_json::to_string(&map).unwrap().as_bytes());
    }
}
