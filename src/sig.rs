use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Sig {
    hsh: HashMap<char, u8>,
    sig: String,
}

impl Sig {
    pub fn for_word(str: &str) -> Sig {
        let lower = str.to_lowercase();
        let mut hsh: HashMap<char, u8> = HashMap::new();
        for ch in lower.chars() {
            *hsh.entry(ch).or_insert(0) += 1;
        }

        let mut chars = hsh.keys().collect::<Vec<_>>();
        chars.sort();

        let mut sig = String::default();
        for ch in chars {
            let piece = format!("{}{}", ch, hsh[ch]);
            sig.push_str(&piece);
        }

        Sig { sig, hsh }
    }

    // Returns true of other can be made from the letters in self.
    pub fn contains(&self, other: &Self) -> bool {
        if other.hsh.len() > self.hsh.len() {
            false
        } else {
            for ch in other.hsh.keys() {
                if !self.hsh.contains_key(ch) {
                    return false;
                }

                // unwrap: safe because we've checked for membership already.
                if other.hsh.get(ch).unwrap() > self.hsh.get(ch).unwrap() {
                    return false;
                }
            }
            true
        }
    }
}

impl PartialEq for Sig {
    fn eq(&self, other: &Self) -> bool {
        self.sig == other.sig
    }
}

impl Eq for Sig {}

impl Hash for Sig {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.sig.hash(hasher);
    }
}

impl Display for Sig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sig:{}", self.sig)
    }
}

#[cfg(test)]
mod test {
    use std::collections::hash_map::DefaultHasher;

    use super::*;

    #[test]
    fn simple_test() {
        // Check the sig string for a word is what is expected.
        let sig = Sig::for_word("astonishment");
        assert_eq!("a1e1h1i1m1n2o1s2t2", sig.sig);
    }

    #[test]
    fn hsh_test() {
        // Ensures that the hash of the sig string is the same as the
        // hash of the sig.
        let sig = Sig::for_word("supercilious");

        let mut hasher1 = DefaultHasher::new();
        sig.sig.hash(&mut hasher1);
        let sig_hash = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        sig.hash(&mut hasher2);
        let hash = hasher2.finish();

        assert_eq!(sig_hash, hash);
    }

    #[test]
    fn eq_test() {
        let sig1 = Sig::for_word("abc");
        let sig2 = Sig::for_word("cba");

        let two_sig1 = Sig::for_word("abca");
        let two_sig2 = Sig::for_word("baca");

        let not_sig1 = Sig::for_word("abd");

        let short_sig1 = Sig::for_word("ab");

        // Everything is eq to itself.
        assert_eq!(sig1, sig1);
        assert_eq!(sig2, sig2);
        assert_eq!(two_sig1, two_sig1);
        assert_eq!(two_sig2, two_sig2);
        assert_eq!(not_sig1, not_sig1);
        assert_eq!(short_sig1, short_sig1);

        // Order doesn't matter
        assert_eq!(sig1, sig2);
        assert_eq!(sig2, sig1);

        // things that are not equal
        assert_ne!(sig1, two_sig1);
        assert_ne!(sig1, not_sig1);
        assert_ne!(sig1, short_sig1);
    }

    #[test]
    fn contains_test() {
        let sig_one = Sig::for_word("a");
        let sig_two = Sig::for_word("ab");
        let sig_three = Sig::for_word("abc");
        let sig_four = Sig::for_word("abcd");

        assert!(sig_four.contains(&sig_four));
        assert!(sig_four.contains(&sig_three));
        assert!(sig_four.contains(&sig_two));
        assert!(sig_four.contains(&sig_one));

        assert!(!sig_three.contains(&sig_four));

        let rep_one = Sig::for_word("aa");
        assert!(rep_one.contains(&sig_one));
        assert!(!sig_one.contains(&rep_one));
    }

    #[test]
    fn elephant() {
        let elephant = Sig::for_word("elephant");
        let ant = Sig::for_word("ant");

        assert!(elephant.contains(&ant));
    }
}
