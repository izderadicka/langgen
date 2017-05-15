extern crate hyper;
use std::io::Write;
use self::hyper::server::{Request, Response};
use self::hyper::header;
use self::hyper::mime::{self, Mime, TopLevel, SubLevel};
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

#[cfg(test)]
mod tests {
    use super::*;

   

    #[test]
    fn qs() {
        let n = extract_qs("/sentence?number=10", "number", 1);
        assert_eq!(n, 10);

    }
}

