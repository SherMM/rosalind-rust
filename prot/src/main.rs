use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn translate_rna_to_protein(rna: String, codons: HashMap<&str, &str>) -> String {
    let mut protein = String::new();
    for i in (0..rna.len()-2).step_by(3) {
    	let codon = &rna[i..i+3];
    	let acid = codons.get(codon).unwrap();
    	if acid == &"Stop" {break;}
    	else {protein.push_str(acid);}
    }
    protein
}


fn main() {
    let codons: HashMap<&str, &str> = [
        ("UUU", "F"),    ("CUU", "L"), ("AUU", "I"), ("GUU", "V"),
        ("UUC", "F"),    ("CUC", "L"), ("AUC", "I"), ("GUC", "V"),
        ("UUA", "L"),    ("CUA", "L"), ("AUA", "I"), ("GUA", "V"),
        ("UUG", "L"),    ("CUG", "L"), ("AUG", "M"), ("GUG", "V"),
        ("UCU", "S"),    ("CCU", "P"), ("ACU", "T"), ("GCU", "A"),
        ("UCC", "S"),    ("CCC", "P"), ("ACC", "T"), ("GCC", "A"),
        ("UCA", "S"),    ("CCA", "P"), ("ACA", "T"), ("GCA", "A"),
        ("UCG", "S"),    ("CCG", "P"), ("ACG", "T"), ("GCG", "A"),
        ("UAU", "Y"),    ("CAU", "H"), ("AAU", "N"), ("GAU", "D"),
        ("UAC", "Y"),    ("CAC", "H"), ("AAC", "N"), ("GAC", "D"),
        ("UAA", "Stop"), ("CAA", "Q"), ("AAA", "K"), ("GAA", "E"),
        ("UAG", "Stop"), ("CAG", "Q"), ("AAG", "K"), ("GAG", "E"),
        ("UGU", "C"),    ("CGU", "R"), ("AGU", "S"), ("GGU", "G"),
        ("UGC", "C"),    ("CGC", "R"), ("AGC", "S"), ("GGC", "G"),
        ("UGA", "Stop"), ("CGA", "R"), ("AGA", "R"), ("GGA", "G"),
        ("UGG", "W"),    ("CGG", "R"), ("AGG", "R"), ("GGG", "G") 
    ].iter().cloned().collect();

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let mut rna = String::new();
    f.read_to_string(&mut rna)
        .expect("unable to read file");

    println!("{}", translate_rna_to_protein(rna, codons));
}
