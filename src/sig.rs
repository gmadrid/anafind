use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Sig {
    hsh: HashMap<char, u8>,
    sig: String,
}

impl Sig {
    fn for_word(str: &str) -> Sig {
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

    fn sig(&self) -> &str {
        &self.sig
    }
}

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
        assert_eq!("a1e1h1i1m1n2o1s2t2", sig.sig());
    }

    #[test]
    fn hsh_test() {
        // Ensures that the hash of the sig string is the same as the
        // hash of the sig.
        let sig = Sig::for_word("supercilious");

        let mut hasher1 = DefaultHasher::new();
        sig.sig().hash(&mut hasher1);
        let sig_hash = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        sig.hash(&mut hasher2);
        let hash = hasher2.finish();

        assert_eq!(sig_hash, hash);
    }
}
