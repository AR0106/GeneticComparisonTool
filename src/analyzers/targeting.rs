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

pub fn find_chi_sites(genome: Vec<u8>) -> Vec<u64> {
    let mut chi_sites: Vec<u64> = Vec::new();

    let chi_id: [u8; 8] = [0o3, 0o2, 0o1, 0o3, 0o3, 0o1, 0o3, 0o3];

    for i in 0..genome.len() - 8 {
        if genome[i..i + 8] == chi_id {
            println!("Chi Site Found at Index {}", i);
            chi_sites.push(i as u64);
        }
    }

    chi_sites
}
