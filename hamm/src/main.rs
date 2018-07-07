use std::env;
use std::fs::File;
use std::io::prelude::*;


fn calculate_hamm_dist(strand1: &str, strand2: &str) -> usize {
	let count = strand1.chars().zip(strand2.chars()).filter(
		|&(strand1, strand2)| strand1 != strand2).count();
	count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let mut dna_strands = String::new();
    f.read_to_string(&mut dna_strands)
        .expect("unable to read file");
    let strands = dna_strands.split("\n").collect::<Vec<&str>>();
    println!("Distance: {}", calculate_hamm_dist(strands[0], strands[1]));
}
