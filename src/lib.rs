use std::io;
use std::io::prelude::*;
use std::path::{Path};
use std::fs::File;

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    StartOfSentence,
    EndOfSentence(char),
    Interpuction(char),
    Quote(char),
}

pub struct FileTokenizer<R: BufRead> {
    file: R,
}

impl FileTokenizer<io::BufReader<File>> {
    pub fn new_from_path(file_path: &Path) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let file = io::BufReader::new(file);
        let tokenizer = FileTokenizer { file };
        Ok(tokenizer)
    }
}

impl<'a> FileTokenizer<&'a[u8]> {
 pub fn new_from_buffer(buf: &'a[u8]) -> Self {
     FileTokenizer { file:buf}
 }
}

#[derive(Debug)]
pub struct FileTokenizerIterator<R: BufRead> {
    file: R,
    current_line: Vec<String>,
}

impl<R: BufRead> IntoIterator for FileTokenizer<R> {
    type Item = Token;
    type IntoIter = FileTokenizerIterator<R>;

    fn into_iter(self) -> Self::IntoIter {
        FileTokenizerIterator {
            file: self.file,
            current_line: vec![],
        }
    }
}

fn tokenize(s:String) -> Token {
    
    Token::Word(s)
}

impl<R: BufRead> Iterator for FileTokenizerIterator<R> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_line.pop() {
            Some(w) => Some(tokenize(w)),
            None => {
                while self.current_line.len() < 1 {
                    let mut s = String::new();
                    
                    if self.file.read_line(&mut s).is_err() || s.len() == 0 {
                        return None;
                    }
                    self.current_line = s.split(' ')
                        .flat_map(|s| s.split(','))
                        .map(|s| s.trim().to_string())
                        .filter(|s| s.len() > 0)
                        .rev()
                        .collect();

                }
                self.current_line.pop().map(tokenize)

            }
        }
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn tokens() {
        let text = "\
        usak kulisak
        novotka

        hey how

        ";

        let t = FileTokenizer::new_from_buffer(text.as_bytes());
        let mut i = t.into_iter();
        assert_eq!(i.next(), Some(Token::Word("usak".to_string())));

        let r:Vec<_> = i.collect();
        assert_eq!(4, r.len());

        assert_eq!(r[3], Token::Word("how".to_string()));
        println!("{:?}", r[2]);
    }

    #[test]
    fn iter() {
        let text = "\
        usak kulisak
        novotka

        hey how

        ";

        let t = FileTokenizer::new_from_buffer(text.as_bytes());
        let mut i = 0;
        for token in t {
            println!("{:?}", token);
            i+=1;
        }
        assert_eq!(i, 5);
    }
}

