use regex::Regex;
use std::{
    borrow::Borrow,
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, Read, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let fna = File::open(&args[1]);
    let mut genomic_data = String::new();

    fna.unwrap().read_to_string(&mut genomic_data);

    let pattern = Regex::new(r">(.*)").unwrap();
    let chromosome_rna: Vec<&str> = pattern.split(&genomic_data.as_str()).collect();
    let chromosome_list: Vec<&str> = pattern
        .find_iter(&genomic_data.as_str())
        .map(|mat| &genomic_data[mat.start()..mat.end()])
        .collect();

    const MAX_LENGTH: i32 = 9;

    for chromosome in 0..chromosome_list.len() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        let genome = &chromosome_rna[chromosome];

        println!("{}/{}", chromosome + 1, chromosome_list.len());

        for i in 2..MAX_LENGTH {
            let mut left = 0;
            let mut right = i;

            for j in 0..genome.len() {
                let mut _genome = &genome[left as usize..right as usize];
                map.entry(_genome.to_owned().replace('\n', "").as_str())
                    .and_modify(|val| *val += 1)
                    .or_insert(1);
            }
        }

        let mut decodedFile = File::create(
            chromosome_list[chromosome]
                .to_owned()
                .replace(">", "")
                .to_string()
                + ".txt",
        );
        decodedFile
            .unwrap()
            .write(serde_json::to_string(&map).unwrap().as_bytes());
    }
}
