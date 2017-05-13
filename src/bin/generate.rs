extern crate rustc_serialize;
extern crate docopt;
extern crate langgen;

use std::path::Path;
use docopt::Docopt;

const USAGE: &'static str = "
Usage: generate [options] <file>

Options:
    -n, --number=<x>  print x random sentences [default = 1]
    -h, --help  display this help and exit
    -v, --version  output version information and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
    flag_number: Option<usize>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    
    let num = args.flag_number.unwrap_or(1);
    let fname = args.arg_file;
    let i=langgen::FileTokenizer::new_from_path(Path::new(&fname)).expect("Cannot open file").into_iter();
    let mut trigrams = langgen::Trigrams::new();
    trigrams.fill(Box::new(i));
    for _i in 0..num {
        println!("{}", trigrams.random_sentence(1000));
    }
    
}