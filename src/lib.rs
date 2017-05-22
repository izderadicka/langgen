extern crate rand;
extern crate fnv;

use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::mem;
use std::collections::{HashMap, BTreeSet, VecDeque};
use std::cmp::Ordering;
use rand::distributions::{IndependentSample, Range as RandomRange};
use fnv::{FnvHashMap};

const BUF_SIZE: usize = 8;

pub mod http_handler;

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    EndOfSentence(char),
    Interpuction(char),
    Quote(char),
    BracketLeft(char),
    BracketRight(char)

}

type MyHashMap<K,V> = FnvHashMap<K,V>;
type MyHashSet<V> = BTreeSet<V>;

pub struct FileTokenizer<R: Read> {
    file: R,
}

impl FileTokenizer<File> {
    pub fn from_path(file_path: &Path) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let tokenizer = FileTokenizer { file };
        Ok(tokenizer)
    }
}

impl<'a, R:Read> FileTokenizer<R> {
    pub fn new(buf: R) -> Self {
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



#[derive(Debug)]
pub struct Trigrams {
    start_words: MyHashMap<u32, u32>,
    all_words: MyHashMap<String, u32>,
    rev_index: Vec<String>,
    trigrams: MyHashMap<u32, MyHashMap<u32, MyHashSet<u32>>>,
}

#[derive(Debug)] 
pub struct TrigramsStats {
    pub start_words: u32,
    pub all_words: u32,
    pub trigrams: u64
}


impl Trigrams {
    pub fn new() -> Self {
        Trigrams {
            start_words: MyHashMap::default(),
            all_words: MyHashMap::default(),
            rev_index: Vec::new(),
            trigrams: MyHashMap::default(),
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

    pub fn stats(&self) -> TrigramsStats {
        let mut count = 0;
        for (_,m) in &self.trigrams {
            for (_, s) in m {
                count+= s.len()
            }
        }
        TrigramsStats{
            all_words: self.all_words.len() as u32,
            start_words: self.start_words.len() as u32,
            trigrams: count as u64
        }

    }

    pub fn print_stats(&self) {
        let stats = self.stats();

        println!("Number of words: {}", stats.all_words);
        println!("Number of starting words: {}", stats.start_words);
        println!("Number of trigrams: {}", stats.trigrams);

    }

    fn random_start_trigram(&self) -> (u32, u32, u32) {
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

    pub fn fill<R:Read>(&mut self, iter: FileTokenizerIterator<R>) {
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
                let current = self.all_words.get(&s).map(|x| *x);
                let w = match current {
                    Some(x) => x,
                    None => {
                        let s2 = s.clone();
                        let k = self.rev_index.len();
                        assert!(k<= std::u32::MAX as usize);
                        let k = k as u32;
                        self.rev_index.push(s2);
                        self.all_words.insert(s,k);
                        k
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

                    let map1 = self.trigrams.entry(w1_copy).or_insert(MyHashMap::default());
                    let map2 = map1.entry(w2).or_insert(MyHashSet::default());
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


    fn get_trigram(&self, w1: u32, w2: u32) -> Option<(u32, u32, u32)>{
        
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

        fn output(map: &Vec<String>, sentence: &mut String, w: u32, first:bool ) {
            let s: &String = &map[w as usize];
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
                output(&self.rev_index, &mut sentence, w1, true );
                output(&self.rev_index,&mut sentence, w2, false );
            }
            output(&self.rev_index,&mut sentence, w3, false );
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

pub fn count_words<R:Read>(iter: FileTokenizerIterator<R> )-> Vec<(String, u64)> {
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
                Ok(mut s) => {
                    s.shrink_to_fit();
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
                        },
                        b'('|b'['|b'{' => self.token = Some(Token::BracketLeft(char::from(ch))),
                        b')'|b']'|b'}' => self.token = Some(Token::BracketRight(char::from(ch))),
                        b',' | b';' => self.token = Some(Token::Interpuction(char::from(ch))),
                        b'"' => self.token = Some(Token::Quote(char::from(ch))),
                        b'\'' if self.word.len() == 0 => self.token=Some(Token::Quote(char::from(ch))),
                        b' ' | b'\n' | b'\t' => {
                            if let  Some(&b'\'') = self.word.last() {
                                let ch = self.word.pop().unwrap();
                                self.token = Some(Token::Quote(char::from(ch)));
                            }
                        }
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

        let t = FileTokenizer::new(text.as_bytes());
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

        let t = FileTokenizer::new(text.as_bytes());
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
        let t = FileTokenizer::new(text.as_bytes());
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
    fn quotes() {
        let text ="(neco), 's cim let's not talk' ";
        let mut count_quote = 0;
        let mut count_bracket =0;
        let mut count_word = 0;

        let t = FileTokenizer::new(text.as_bytes());
        for token in t {
            use Token::*;
            println!("Token: {:?}", token);
            match token {
                Word(_) => count_word+=1,
                BracketLeft(_)|BracketRight(_) => count_bracket+=1,
                Quote(_) => count_quote+=1,
                _ => ()
            }
        }

        assert_eq!((2,2,6), (count_quote, count_bracket, count_word))
    }

    #[test]
    fn quote() {
        let text = "\"kulisak\"";
        let mut i = FileTokenizer::new(text.as_bytes()).into_iter();
        assert_eq!(i.next(), Some(Token::Quote('\"')));
        assert_eq!(i.next(), Some(Token::Word("kulisak".to_string())));
        assert_eq!(i.next(), Some(Token::Quote('\"')));
    }

    #[test]
    fn freq() {
        let text = "say hello
        hello you must say,
        to all good men";
        let i = FileTokenizer::new(text.as_bytes()).into_iter();
        let v = count_words(i);

        assert_eq!(v[0], ("hello".to_string(), 2));
        assert_eq!(v[0], ("hello".to_string(), 2));

    }

    

    fn gen_trigrams() -> Trigrams {
        let text = "Say hello
        hello you must say,
        to all good men.
        Do'nt say hello to bad men!";
        let i = FileTokenizer::new(text.as_bytes()).into_iter();
        let mut trigrams = Trigrams::new();
        trigrams.fill(i);
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

