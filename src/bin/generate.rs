extern crate rustc_serialize;
extern crate docopt;
extern crate langgen;

use std::path::Path;
use docopt::Docopt;

const USAGE: &'static str = "
Usage: generate [options] <file>...

Options:
    -n, --number=<x>  print x random sentences [default = 1]
    -s, --stats  print language courpus stats
    -h, --help  display this help and exit
    -v, --version  output version information and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: Vec<String>,
    flag_number: Option<usize>,
    flag_stats: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    
    let num = args.flag_number.unwrap_or(1);
    let fnames = args.arg_file;
    let mut trigrams = langgen::Trigrams::new();
    for fname in fnames {
        let i=langgen::FileTokenizer::from_path(Path::new(&fname)).expect("Cannot open file").into_iter();
        trigrams.fill(i);
    }
    if args.flag_stats {
        trigrams.print_stats();
        return
    }
    for _i in 0..num {
        println!("{}", trigrams.random_sentence(1000));
    }
    
}