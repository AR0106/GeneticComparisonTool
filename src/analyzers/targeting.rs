use crate::gcto_file::PARSER_DICTIONARY;

pub fn find_pam_sites(genome: Vec<u8>) -> Vec<u64> {
    let mut pam_sites: Vec<u64> = Vec::new();

    for i in 1..genome.len() - 1 {
        if genome[i] == PARSER_DICTIONARY[&'G'] && genome[i + 1] == PARSER_DICTIONARY[&'G'] {
            println!("{}GG PAM Site Found at Index {}", genome[i - 1], i);
            pam_sites.push(i as u64);
        }
    }

    pam_sites
}

