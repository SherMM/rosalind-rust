use std::env;
use std::fs::File;
use std::io::prelude::*;

fn rev_comp(dna: String) -> String {
	let mut rvc = String::new();
	for nucleotide in dna.chars().rev() {
		let comp = match nucleotide {
			'A' => 'T',
			'T' => 'A',
			'C' => 'G',
			 _  => 'C'
		};
		rvc.push(comp);
	}
	rvc
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let mut dna = String::new();
    f.read_to_string(&mut dna)
        .expect("unable to read file");
    dna = dna.trim().to_string();
    println!("REVC: {}", rev_comp(dna));
}
