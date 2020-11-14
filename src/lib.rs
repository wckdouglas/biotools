extern crate pyo3;
extern crate bio;
extern crate flate2;
use std::string::String;
use std::io::BufReader;
use std::fs;
use bio::io::fastq;
use bio::io::fastq::FastqRead;
use pyo3::prelude::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use flate2::bufread;

fn get_fastq_reader(path: &String) -> Box<::std::io::Read> {
    // borrowed from 
    // https://github.com/sndrtj/fastq-count/blob/master/src/main.rs
    if path.ends_with(".gz") {
        let f = fs::File::open(path).unwrap();
        Box::new(bufread::MultiGzDecoder::new(BufReader::new(f)))
    } else {
        Box::new(fs::File::open(path).unwrap())
    }
}

#[pyfunction]
fn fq_stat(filename: String) -> PyResult<(usize, usize)>{
	let reader = fastq::Reader::new(get_fastq_reader(&filename));
    let mut basecount = 0;
    let mut readcount = 0;
	for result in reader.records(){
        let record = result.expect("Error during fastq record parsing");
        let seq = record.seq();
        let len = seq.len();
        basecount += len;
        readcount += 1;
    }
	Ok((readcount, basecount))
}


/// A Python module implemented in Rust.
#[pymodule]
fn biotools_lib(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(fq_stat))?;
    Ok(())
}
