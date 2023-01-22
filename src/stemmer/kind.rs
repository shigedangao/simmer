use super::ParsedWord;

// Constant
const CONSONENT_LIST: [char; 20] = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'];

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Kind {
    Consonent,
    Vowel,
    None
}

impl From<char> for Kind {
    fn from(c: char) -> Self {
        if CONSONENT_LIST.contains(&c) {
            return Self::Consonent;
        }

        Self::Vowel
    }
}

impl From<ParsedWord> for Kind {
    fn from(p: ParsedWord) -> Self {
        match p {
            ParsedWord::C(_) => Kind::Consonent,
            ParsedWord::V(_) => Kind::Vowel,
            ParsedWord::None => Kind::None
        }
    }
}