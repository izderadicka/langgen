extern crate langgen;
extern crate hyper;
extern crate rustc_serialize;
extern crate docopt;
extern crate zip;

use langgen::http_handler::generate_sentence;
use docopt::Docopt;
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method;
use self::hyper::mime::{self, Mime, TopLevel, SubLevel};
use self::hyper::header;
use std::io::{Write, stderr, copy};
use std::path::{Path};
use std::fs::File;

const USAGE: &'static str = "
Usage: serve [options] <file_or_zip>...

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

fn serve(server: Server, trigrams: langgen::Trigrams, index_file:String) {

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
            if path.len()>=9 && path[..9]  == "/sentence"[..] {
                generate_sentence(&req,res, &trigrams);
            } else {
                if let Ok(mut f) = File::open(&index_file) {
                    let size = f.metadata().unwrap().len();
                    {
                    let headers= res.headers_mut();
                    headers.set(header::ContentType(Mime(TopLevel::Text,
                                             SubLevel::Html,
                                             vec![(mime::Attr::Charset, mime::Value::Utf8)])));
                    headers.set(header::ContentLength(size));
                    }
                    let mut res = res.start().unwrap();
                    copy(&mut f,&mut res).unwrap();
                    
                } else {
                *res.status_mut() = StatusCode::InternalServerError;
                }
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
        let i=langgen::FileTokenizer::new_from_path(Path::new(&fname)).expect("Cannot open file").into_iter();
        trigrams.fill(Box::new(i));
    }
    let full_addr = format!("{}:{}", addr, port);
    let server=Server::http(&full_addr).unwrap();
    println!("SERVING NOW ON {}", full_addr);
    serve(server, trigrams, index_file);
    
}