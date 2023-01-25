use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::env;

fn main() {
    let vec_1 = single_row_csv_to_vec("a.csv");
    let vec_2 = single_row_csv_to_vec("b.csv");

    for line in vec_1 {
        if vec_2.contains(&line){
            println!("Matched: {}", line);
        }
    }
}

fn single_row_csv_to_vec (s: &str)->Vec<String> {
    let reader = BufReader::new(File::open(s).expect("Could not open the file"));
    let mut vec = Vec::new();

    for line in reader.lines() {
        let ln = line.unwrap();
        if &ln != "" {
            vec.push(ln);
        }
    }

    return vec;
}
