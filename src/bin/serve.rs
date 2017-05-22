extern crate langgen;
extern crate hyper;
extern crate rustc_serialize;
extern crate docopt;
extern crate zip;

use langgen::http_handler::serve;
use langgen::{FileTokenizer};
use docopt::Docopt;
use std::io::{Write, stderr};
use std::path::{Path};
use zip::ZipArchive;
use std::fs::File;


const USAGE: &'static str = "
Usage: serve [options] <file>...

File parameters are either plain text files, or zipped text files
Options:
    -p, --port=<x>  listen on this port [default = 8088]
    -a, --addr=<h>  listen on this local address [defalt = 127.0.0.1] 
    -i, --index=<i>  index file [default=./web/index.html]
    -h, --help  display this help and exit
    -v, --version  output version information and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: Vec<String>,
    flag_port: Option<u16>,
    flag_addr: Option<String>,
    flag_index: Option<String>
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    
    let port = args.flag_port.unwrap_or(8088);
    let addr = args.flag_addr.unwrap_or("127.0.0.1".to_string());
    let fnames = args.arg_file;
    let default_index = "./web/index.html".to_string();
    let index_file = args.flag_index.unwrap_or(default_index);
    ;
    if ! Path::new(&index_file).is_file() {
        write!(stderr(), "Missing index file").unwrap();
        std::process::exit(1);
    }
    let mut trigrams = langgen::Trigrams::new();
    for fname in fnames {
        let p = Path::new(&fname);
        match p.extension().and_then(|e| e.to_str()) {
            Some("zip") => {
                let f = File::open(p).unwrap();
                let mut archive= ZipArchive::new(f).unwrap();
                for i in 0..archive.len() {
                    let zip_file = archive.by_index(i).unwrap();
                    let i = FileTokenizer::new(zip_file).into_iter();
                    trigrams.fill(i);

                }
            },
            _ => {
                let i=FileTokenizer::from_path(p).expect("Cannot open file").into_iter();
                trigrams.fill(i);
            }
        }
        
    }
    assert!(trigrams.stats().start_words>0);
    let full_addr = format!("{}:{}", addr, port);
    println!("SERVING NOW ON {}", full_addr);
    serve(&full_addr, trigrams, index_file);
    
}