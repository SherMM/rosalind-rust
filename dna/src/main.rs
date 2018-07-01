use std::env;
use std::fs::File;
use std::io::prelude::*;


fn count_nucleotides(dna: String) -> [u64; 4] {
    let mut counts: [u64; 4] = [0; 4];
    for nucleotide in dna.chars() {
        match nucleotide {
            'A' => counts[0] += 1,
            'C' => counts[1] += 1,
            'G' => counts[2] += 1,
            'T' => counts[3] += 1,
            _ => println!("unknown char")
        }
    }
    counts
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let mut dna = String::new();
    f.read_to_string(&mut dna)
        .expect("unable to read file");

    println!("Counts:\n{:?}", count_nucleotides(dna));
}
