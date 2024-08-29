use phf::{phf_map, PhfHash};
use std::{
    fs::File,
    io::{Read, Write},
};

static WEB_NUCLEOTIDE: phf::Map<char, &str> = phf_map! {
    'a' => "<div class='a'>A</div>",
    't' => "<div class='t'>T</div>",
    'c' => "<div class='c'>C</div>",
    'g' => "<div class='g'>G</div>",
    'A' => "<div class='a'>A</div>",
    'T' => "<div class='t'>T</div>",
    'C' => "<div class='c'>C</div>",
    'G' => "<div class='g'>G</div>",
    'n' => "<div class='xn'>N</div>",
    'x' => "<div class='xn'>x</div>",
    'N' => "<div class='xn'>N</div>",
    'X' => "<div class='xn'>x</div>",
};

fn sequence_to_html(sequence: Vec<char>) -> String {
    let mut html = String::new();
    for base in sequence {
        let base = base;
        html.push_str(WEB_NUCLEOTIDE[&base]);
    }
    html
}

pub struct ComparisonData {
    pub sequence1: Vec<char>,
    pub sequence2: Vec<char>,
    pub title: String,
}

pub fn generate_visual_comparison(comp_data: ComparisonData) {
    let sequence1 = sequence_to_html(comp_data.sequence1);
    let sequence2 = sequence_to_html(comp_data.sequence2);

    let mut html: String = include_str!("web-comp.html").to_string();

    html = html
        .replace("{comp-data-1}", &sequence1)
        .replace("{comp-data-2}", &sequence2)
        .replace("{comp-name}", &comp_data.title);

    File::create(comp_data.title + ".html")
        .unwrap()
        .write(html.as_bytes());

    println!("{}", html);
}
