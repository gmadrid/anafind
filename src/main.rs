use anyhow::Error;
use argh::FromArgs;
use fehler::throws;
use sig::Sig;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

mod sig;

const WORDS_FILE: &str = "/usr/share/dict/words";

type HashType = HashMap<Sig, Vec<String>>;

#[derive(FromArgs)]
/// Find words in a the input string.
struct AnafindArgs {
    #[argh(positional)]
    pub pattern: String,

    /// output words of this length
    #[argh(option, short = 'l')]
    pub length: Option<u8>,
}

#[throws]
fn open_words_file() -> HashType {
    let rdr = std::fs::File::open(WORDS_FILE)?;
    let bufrdr = BufReader::new(rdr);

    let mut hsh: HashType = HashType::default();

    for word in bufrdr.lines() {
        let word = word?;
        let sig = Sig::for_word(&word);

        hsh.entry(sig)
            .or_insert_with(Vec::default)
            .push(word.to_lowercase());
    }
    hsh
}

// We want to show all words that can be make from the input set of letters.
// That means we want all words that are a subset of the pattern.

#[throws]
fn main() {
    let args = argh::from_env::<AnafindArgs>();
    let pattern_sig = Sig::for_word(&args.pattern);

    let hsh = open_words_file()?;

    let mut found = Vec::default();
    for (sig, v) in hsh.iter() {
        if pattern_sig.contains(sig) {
            for found_word in v {
                if found_word.len() > 2 {
                    if args.length.is_some() && args.length.unwrap() as usize != found_word.len() {
                        continue;
                    }
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
