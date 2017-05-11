extern crate rustc_serialize;
extern crate docopt;
extern crate langgen;

use std::path::Path;
use docopt::Docopt;

const USAGE: &'static str = "
Usage: count [options] [<file>]

Options:
    -l, --limit=<x>  print first x most frequest words [default = 20]
    -h, --help  display this help and exit
    -v, --version  output version information and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: Option<String>,
    flag_limit: Option<usize>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    
    let lines = args.flag_limit.unwrap_or(20);
    let fname = args.arg_file.expect("file name missing");
    let i=langgen::FileTokenizer::new_from_path(Path::new(&fname)).expect("Cannot open file").into_iter();
    
    let res  = langgen::count_words(Box::new(i));
    for (word, count) in res.into_iter().take(lines) {
        println!("{} : {}", word, count);
    }
    
}