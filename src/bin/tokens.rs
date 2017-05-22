extern crate langgen;
use langgen::FileTokenizer;
use std::path::Path;

fn main() {
    let fname = std::env::args().nth(1).expect("Missing arg");
    let t=FileTokenizer::from_path(Path::new(&fname)).expect("Cannot Open");
    for token in t {
    println!("{:?}", token);
    }
}