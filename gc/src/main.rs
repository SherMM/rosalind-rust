use std::env;
use std::fs::File;
use std::io::{BufReader,BufRead};

fn calculate_gc_content(strand: String) -> f64 {
	let mut gc = 0.0;
	let mut size = 0.0;
	for nucleotide in strand.chars() {
		if nucleotide == 'G' || nucleotide == 'C' {
			gc += 1.0;
			size += 1.0;
		} else {
			size += 1.0;
		}
	}
	gc / size
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).expect("file not found");
    let mut code = String::from("");
    let mut strand = String::new();
    let mut max_code = String::from("");
    let mut max_gc = 0.0;
    for line in BufReader::new(file).lines() {
    	let l = line.unwrap();
    	if l.starts_with(">") {
    		if strand.len() != 0 {
    			let gc = calculate_gc_content(strand);
    			// update max gc content & fasta code
    			if gc >= max_gc {
    				max_gc = gc;
    				max_code = code.clone();
    			}
    			strand = String::new();
    		}
			code = l;
    	} else {
    		strand.push_str(&l);
    	}
    }
    // make sure last strand has calculate gc content
    let gc = calculate_gc_content(strand);
    if gc >= max_gc {
    	max_gc = gc;
    	max_code = code.clone();
    }
    println!("{}", &max_code[1..max_code.len()]);
    println!("{}", max_gc*100.0);
}
