extern crate hyper;
use std::io::{Write, copy};
use std::fs::File;
use self::hyper::server::{Server, Request, Response};
use self::hyper::header;
use self::hyper::mime::{self, Mime, TopLevel, SubLevel};
use self::hyper::method::Method;
use self::hyper::status::StatusCode;
use super::Trigrams;
use self::hyper::Url;
use std::str::FromStr;

fn extract_qs<T:FromStr+Copy>(path: &str, name: &str, default: T) -> T {
    let mut url="http:".to_string();
    url+=path;
    Url::parse(&url)
        .ok()
        .and_then(|url| {
                      url.query_pairs()
                          .find(|pair| pair.0 == name)
                          .map(|n| T::from_str(&n.1).unwrap_or(default))
                  })
        .unwrap_or(default)
}
pub fn generate_sentence(req: &Request, mut res: Response, trigrams: &Trigrams) {

    let n = match &req.uri {
        &self::hyper::uri::RequestUri::AbsolutePath(ref path) => extract_qs(&path, "number", 1),
        _ => 1,
    };
    let mut quote = String::new();
    for i in 0..n {
        let sentence = trigrams.random_sentence(1000);
        quote += &sentence;
        if i < n - 1 {
            quote += " "
        }
    }

    let quote = quote.into_bytes();
    {
        let headers = res.headers_mut();
        headers.set(header::ContentType(Mime(TopLevel::Text,
                                             SubLevel::Plain,
                                             vec![(mime::Attr::Charset, mime::Value::Utf8)])));
        headers.set(header::ContentLength(quote.len() as u64));
    }
    let mut res = res.start().unwrap();
    res.write_all(&quote).unwrap();

}

pub fn serve(addr: &str, trigrams: Trigrams, index_file:String) {
let server=Server::http(addr).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

   

    #[test]
    fn qs() {
        let n = extract_qs("/sentence?number=10", "number", 1);
        assert_eq!(n, 10);

    }
}

