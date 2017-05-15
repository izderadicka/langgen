extern crate langgen;
extern crate hyper;
extern crate rustc_serialize;
extern crate docopt;

use langgen::http_handler::generate_sentence;
use std::path::Path;
use docopt::Docopt;
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method;
use self::hyper::header;
use std::io::Write;

const USAGE: &'static str = "
Usage: serve [options] <file>...

Options:
    -p, --port=<x>  listen on this port [default = 8088]
    -a, --addr=<h>  listen on this local address [defalt = 127.0.0.1] 
    -h, --help  display this help and exit
    -v, --version  output version information and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: Vec<String>,
    flag_port: Option<u16>,
    flag_addr: Option<String>,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    
    let port = args.flag_port.unwrap_or(8088);
    let addr = args.flag_addr.unwrap_or("127.0.0.1".to_string());
    let fnames = args.arg_file;
    let mut trigrams = langgen::Trigrams::new();
    for fname in fnames {
        let i=langgen::FileTokenizer::new_from_path(Path::new(&fname)).expect("Cannot open file").into_iter();
        trigrams.fill(Box::new(i));
    }
    let full_addr = format!("{}:{}", addr, port);
    let server=Server::http(&full_addr).unwrap();
    println!("SERVING NOW ON {}", full_addr);
    let _l = server.handle(move |req:Request, mut res:Response| {
    {
        // Allow CORS
        let headers= res.headers_mut();
        headers.set(header::AccessControlAllowOrigin::Any);
        headers.set(header::AccessControlAllowMethods(vec![Method::Get]))
    }
    match req.method {
        Method::Get => {
            if let &self::hyper::uri::RequestUri::AbsolutePath(ref path) =&req.uri {
            if path[..9]  == "/sentence"[..] {
                generate_sentence(&req,res, &trigrams);
            } else {
                *res.status_mut() = StatusCode::NotFound;
            }
            } else {
                *res.status_mut() = StatusCode::BadRequest;
            }
        },
        _ => {
            *res.status_mut() = StatusCode::MethodNotAllowed;
            let mut res = res.start().unwrap();
            res.write_all(b"Invalid Request!").unwrap();
        }
    }
    });
}