use anyhow::Error;
use fehler::throws;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

mod sig;

const WORDS_FILE: &str = "/usr/share/dict/words";

type HashType = HashMap<String, Vec<String>>;

fn make_alpha_anagram(word: &str) -> String {
    let mut chars = word.chars().collect::<Vec<_>>();
    chars.sort();
    chars.into_iter().collect::<String>()
}

fn make_word_set(word: &str) -> HashSet<char> {
    word.chars().collect()
}

#[throws]
fn open_words_file() -> HashType {
    let rdr = std::fs::File::open(WORDS_FILE)?;
    let bufrdr = BufReader::new(rdr);

    let mut hsh: HashType = HashType::default();

    for word in bufrdr.lines() {
        let word = word?;
        let anagram = make_alpha_anagram(&word);

        hsh.entry(anagram)
            .or_insert(Vec::default())
            .push(word.to_owned());
    }
    hsh
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let pattern_set = make_word_set(&pattern);

    let hsh = open_words_file().unwrap();

    let mut found = Vec::default();
    for (k, v) in hsh.iter() {
        let key_set = make_word_set(k);
        if key_set.is_subset(&pattern_set) {
            for found_word in v {
                if found_word.len() <= pattern.len() && found_word.len() > 2 {
                    found.push(found_word);
                }
            }
        }
    }

    found.sort();

    for v in found {
        println!("{}", v);
    }
}
