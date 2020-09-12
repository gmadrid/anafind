pub struct Pattern(String);

impl Pattern {
    pub fn matches(&self, s: &str) -> bool {
        if self.0.len() != s.len() {
            false
        } else {
            for (l, r) in self.0.chars().zip(s.chars()) {
                if l != '.' && l != r {
                    return false;
                }
            }
            true
        }
    }
}

impl From<&str> for Pattern {
    fn from(s: &str) -> Self {
        Pattern(s.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_dots() {
        let p: Pattern = "melancholy".into();

        assert!(p.matches("melancholy"));
        assert!(!p.matches("melancholyy"));
        assert!(!p.matches("melanchol"));
    }

    #[test]
    fn one_dot() {
        assert!(Pattern::from(".ots").matches("dots"));
        assert!(Pattern::from("d.ts").matches("dots"));
        assert!(Pattern::from("do.s").matches("dots"));
        assert!(Pattern::from("dot.").matches("dots"));
    }

    #[test]
    fn dots() {
        assert!(Pattern::from("n.t.gl.b").matches("notaglob"));
    }
}
