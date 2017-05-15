extern crate rand;


use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::mem;
use std::collections::{HashMap, VecDeque, HashSet};
use std::cmp::Ordering;
use std::rc::Rc;
use rand::distributions::{IndependentSample, Range as RandomRange};
use std::borrow::Borrow;

const BUF_SIZE: usize = 8;

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    StartOfSentence,
    EndOfSentence(char),
    Interpuction(char),
    Quote(char),
}

pub struct FileTokenizer<R: Read> {
    file: R,
}

impl FileTokenizer<File> {
    pub fn new_from_path(file_path: &Path) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let tokenizer = FileTokenizer { file };
        Ok(tokenizer)
    }
}

impl<'a> FileTokenizer<&'a [u8]> {
    pub fn new_from_buffer(buf: &'a [u8]) -> Self {
        FileTokenizer { file: buf }
    }
}

pub struct FileTokenizerIterator<R: Read> {
    file: R,
    buff: [u8; BUF_SIZE],
    word: Vec<u8>,
    read: usize,
    curr: usize,
    token: Option<Token>,
}

impl<R: Read> IntoIterator for FileTokenizer<R> {
    type Item = Token;
    type IntoIter = FileTokenizerIterator<R>;

    fn into_iter(self) -> Self::IntoIter {
        FileTokenizerIterator {
            file: self.file,
            buff: [0; BUF_SIZE],
            word: vec![],
            read: 0,
            curr: 0,
            token: None,
        }
    }
}



type RString = Rc<String>;

#[derive(Debug)]
pub struct Trigrams {
    start_words: HashMap<RString, u64>,
    all_words: HashSet<RString>,
    trigrams: HashMap<RString, HashMap<RString, HashSet<RString>>>,
}


impl Trigrams {
    pub fn new() -> Self {
        Trigrams {
            start_words: HashMap::new(),
            all_words: HashSet::new(),
            trigrams: HashMap::new(),
        }
    }
    

    pub fn print_trigrams(&self) {
        for (w1,m) in &self.trigrams {
            for (w2, s) in m {
                for w3 in s {
                    println!("{} {} {}", w1, w2, w3)
                }
            }
        }
    }

    pub fn print_stats(&self) {
        let mut count = 0;
        for (_,m) in &self.trigrams {
            for (_, s) in m {
                count+= s.len()
            }
        }

        println!("Number of words: {}", self.all_words.len());
        println!("Number of starting words: {}", self.start_words.len());
        println!("Number of trigrams: {}", count);

    }

    fn random_start_trigram(&self) -> (RString, RString, RString) {
        let r = random_selection(self.start_words.len());
        let w1 =  self.start_words.keys().nth(r).unwrap().clone();
        let m = self.trigrams.get(&w1).unwrap();
        let r = random_selection(m.len());
        let w2 = m.keys().nth(r).unwrap().clone();
        let s = m.get(&w2).unwrap();
        let r = random_selection(s.len());
        let w3 = s.iter().nth(r).unwrap().clone();
        (w1, w2, w3)
        
    }

    pub fn fill(&mut self, iter: Box<Iterator<Item = Token>>) {
        let mut current_words = VecDeque::new();
        let mut end_sentence = false;
        let mut start_sentence = true;

        for token in iter {
            let s = match token {

                Token::Word(s) => Some(s),
                Token::EndOfSentence(mark) => {
                    end_sentence = true;
                    Some(mark.to_string())
                },
                Token::Interpuction(_) => {
                    /* for now ignore iterpuctions in middle of sentence
                    end_sentence = true;
                    Some(",".to_string())
                    */
                    None
                }
                _ => None

            };
            // construct trigrams
            if let Some(s) = s {
                let current = self.all_words.get(&s)
                    .map(|c| c.clone());
                let w = match current {
                    Some(x) => x,
                    None => {
                        let n = Rc::new(s);
                        let w = n.clone();
                        self.all_words.insert(n);
                        w
                    }
                };

                current_words.push_back(w);
                if current_words.len()== 3 {

                    let w1 = current_words.pop_front().unwrap();
                    let w1_copy = w1.clone();
                    let w2 = current_words[0].clone();
                    let w3 = current_words[1].clone();

                    if start_sentence {
                        let curr= self.start_words.entry(w1).or_insert(0);
                        *curr+=1;
                        start_sentence=false
                    }

                    let map1 = self.trigrams.entry(w1_copy).or_insert(HashMap::new());
                    let map2 = map1.entry(w2).or_insert(HashSet::new());
                    map2.insert(w3);

                }

                if end_sentence {
                    current_words.clear();
                    start_sentence=true;
                    end_sentence=false;
                }

            }
        }
        
    }


    fn get_trigram(&self, w1: RString, w2: RString) -> Option<(RString, RString, RString)>{
        
        match self.trigrams.get(&w1) {
            None => None,
            Some(m) => {
                match m.get(&w2) {
                    None => None,
                    Some(s) => {
                        let r = random_selection(s.len());
                        let w3 = s.iter().nth(r).unwrap().clone();
                        Some((w1, w2, w3))
                    }
                }
            }
        }
        
    }

    pub fn random_sentence(&self, max_len:usize) -> String{
        assert!(max_len>=3);
        let mut sentence = String::new();
        let mut trigram = Some(self.random_start_trigram());
        let mut count = 0;

        fn output(sentence: &mut String, w: &RString, first:bool ) {
            let s: &String = w.borrow();
            match  &s[..] {
                "."|"?"|"!"|","|";" => {
                    sentence.push_str(s)
                },
                _ if first => {
                    sentence.push_str(s)
                }
                _ => {
                    sentence.push_str(" ");
                    sentence.push_str(s);
                }
            }
        }

        while let Some((w1,w2,w3)) = trigram.take() {
            if count == 0 {
                output(&mut sentence, &w1, true );
                output(&mut sentence, &w2, false );
            }
            output(&mut sentence, &w3, false );
            count+=1;
            if count> max_len - 3 {
                break
            }
            trigram =  self.get_trigram(w2, w3) ;
        }

        sentence
    }
  
}

fn random_selection(len: usize) -> usize {
    let mut rng = rand::thread_rng();
    RandomRange::new(0,len).ind_sample(&mut rng)

}

pub fn count_words(iter: Box<Iterator<Item = Token>>) -> Vec<(String, u64)> {
    let mut map: HashMap<String, u64> = HashMap::new();
    let mut res: Vec<(String, u64)>;
    for token in iter {
        match token {
            Token::Word(s) => {
                let mut entry = map.entry(s).or_insert(0);
                *entry += 1;
            }
            _ => (),
        }
    }
    res = map.into_iter().collect();
    res.sort_by(|a, b| match b.1.cmp(&a.1) {
                    Ordering::Equal => a.0.cmp(&b.0),
                    o => o,
                });
    res
}

impl<R: Read> FileTokenizerIterator<R> {
    fn take_word(&mut self) -> Option<Token> {
        if self.word.len() > 0 {
            let word = mem::replace(&mut self.word, vec![]);
            match String::from_utf8(word) {
                Ok(s) => {
                    let word = s;
                    self.word.clear();
                    return Some(Token::Word(word));
                }
                Err(e) => writeln!(io::stderr(), "UTF8 error {}", e).unwrap(),
            }
        }
        None
    }
}
impl<R: Read> Iterator for FileTokenizerIterator<R> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.token.take() {
            token @ Some(_) => return token,
            None => {
                loop {
                    if self.read == self.curr {
                        match self.file.read(&mut self.buff) {
                            Ok(read) => {
                                if read == 0 {
                                    return self.take_word();
                                }

                                self.read = read;
                                self.curr = 0;
                            }
                            Err(e) => {
                                writeln!(io::stderr(), "read error {}", e).unwrap();
                                return None;
                            }
                        }
                    }
                    let ch = self.buff[self.curr];
                    self.curr += 1;
                    //println!("XXX {} {}", self.curr, ch);
                    match ch {
                        b'.' | b'!' | b'?' => {
                            self.token = Some(Token::EndOfSentence(char::from(ch)))
                        }
                        b',' | b';' => self.token = Some(Token::Interpuction(char::from(ch))),
                        b'"' => self.token = Some(Token::Quote(char::from(ch))),
                        b' ' | b'\n' | b'\t' => {}
                        _ => {
                            self.word.push(ch);
                            continue;
                        }
                    }

                    if let w @ Some(_) = self.take_word() {
                        return w;
                    } else if self.token.is_some() {
                        return self.token.take();
                    }
                }
            }
        }

    }
}
// ########################## TESTS #############################################
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn tokens() {
        let text = "\
        usak kulisak
        novotka

        hey how";

        let t = FileTokenizer::new_from_buffer(text.as_bytes());
        let mut i = t.into_iter();
        assert_eq!(i.next(), Some(Token::Word("usak".to_string())));

        let r: Vec<_> = i.collect();
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
            i += 1;
        }
        assert_eq!(i, 5);
    }

    #[test]
    fn seps() {
        let text = "solich; kulich,usak \"kulisak\" stop.";
        let t = FileTokenizer::new_from_buffer(text.as_bytes());
        #[derive(PartialEq,Eq, Debug)]
        struct Count {
            word: usize,
            itp: usize,
            quote: usize,
            stop: usize,
        }

        let mut count = Count {
            word: 0,
            itp: 0,
            quote: 0,
            stop: 0,
        };
        for token in t {
            use Token::*;
            println!("Token: {:?}", token);
            match token {
                Word(_) => count.word += 1,
                Quote(_) => count.quote += 1,
                Interpuction(_) => count.itp += 1,
                EndOfSentence(_) => count.stop += 1,
                _ => (),

            }
        }

        assert_eq!(count,
                   Count {
                       word: 5,
                       itp: 2,
                       quote: 2,
                       stop: 1,
                   });
    }

    #[test]
    fn quote() {
        let text = "\"kulisak\"";
        let mut i = FileTokenizer::new_from_buffer(text.as_bytes()).into_iter();
        assert_eq!(i.next(), Some(Token::Quote('\"')));
        assert_eq!(i.next(), Some(Token::Word("kulisak".to_string())));
        assert_eq!(i.next(), Some(Token::Quote('\"')));
    }

    #[test]
    fn freq() {
        let text = "say hello
        hello you must say,
        to all good men";
        let i = FileTokenizer::new_from_buffer(text.as_bytes()).into_iter();
        let v = count_words(Box::new(i));

        assert_eq!(v[0], ("hello".to_string(), 2));
        assert_eq!(v[0], ("hello".to_string(), 2));

    }

    fn gen_trigrams() -> Trigrams {
        let text = "Say hello
        hello you must say,
        to all good men.
        Do'nt say hello to bad men!";
        let i = FileTokenizer::new_from_buffer(text.as_bytes()).into_iter();
        let mut trigrams = Trigrams::new();
        trigrams.fill(Box::new(i));
        trigrams
    }

    #[test]
    fn trigrams() {
    let trigrams = gen_trigrams();
        println!("{:?}", trigrams);
        assert_eq!(2, trigrams.start_words.len());
        trigrams.print_trigrams();
    }

    #[test]
    fn random_start() {
        let trigrams = gen_trigrams();
        for _i in 0..10 {
            let t = trigrams.random_start_trigram();
            println!("{:?}", t);
        }
    }

    #[test]
    fn sentences() {
        let trigrams = gen_trigrams();
        for _i in 0..10 {
            let s = trigrams.random_sentence(100);
            println!("{}", s);
        }
    }
}

