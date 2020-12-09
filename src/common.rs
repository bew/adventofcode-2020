use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Unable to open given file");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| { line.expect("Unable to read line") })
        .collect()
}
