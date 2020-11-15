use std::string::String;
use flate2::bufread;
use std::io::BufReader;
use std::fs;


pub fn substring(seq: &str, start: usize, end: usize) -> String {
    let seq_bases: Vec<char> = seq.chars().collect();
    let mut substr: String = String::from("");
    for i in start..end {
        substr.push(seq_bases[i]);
    }
    substr
}


pub fn get_fastq_reader(path: &String) -> Box<::std::io::Read> {
    // borrowed from 
    // https://github.com/sndrtj/fastq-count/blob/master/src/main.rs
    if path.ends_with(".gz") {
        let f = fs::File::open(path).unwrap();
        Box::new(bufread::MultiGzDecoder::new(BufReader::new(f)))
    } else {
        Box::new(fs::File::open(path).unwrap())
    }
}
