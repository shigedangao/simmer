use super::porter::ParsedWord;

// Constant
const CONSONENT_LIST: [char; 20] = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'];
const VOWEL_LIST: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Kind {
    Consonent,
    Vowel,
    None
}

impl Kind {
    /// Return whether a word has a vowel (*v*)
    ///
    /// # Arguments
    ///
    /// * `word` - &str
    pub fn has_vowel(word: &str) -> bool {
        for vowel in VOWEL_LIST {
            if word.contains(vowel) {
                return true
            }
        }

        false
    }

    /// Check whether a word end with a double consonent (*d)
    ///
    /// # Arguments
    ///
    /// * `word` - &str
    pub fn end_with_double_consonent(word: &str) -> bool {
        // get the last two char of the word
        let two_end_character = word.get(word.len() - 2..);
        if let Some(end) = two_end_character {
            // split the two chacter into char
            let chars: Vec<char> = end.chars()
                .filter(|c| CONSONENT_LIST.contains(c))
                .collect();

            if let (Some(first), Some(second)) = (chars.first(), chars.get(1)) {
                if first == second {
                    return true;
                }
            }
        }

        false
    }
}

impl From<char> for Kind {
    fn from(c: char) -> Self {
        if CONSONENT_LIST.contains(&c) {
            return Self::Consonent;
        }

        Self::Vowel
    }
}

impl From<&ParsedWord> for Kind {
    fn from(p: &ParsedWord) -> Self {
        match p {
            ParsedWord::C(_) => Kind::Consonent,
            ParsedWord::V(_) => Kind::Vowel,
            ParsedWord::None => Kind::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_have_double_consonents() {
        let word = "ownn";
        let res = Kind::end_with_double_consonent(word);

        assert_eq!(res, true);
    }

    #[test]
    fn expect_to_not_have_double_consonents() {
        let word = "hello";
        let res = Kind::end_with_double_consonent(word);

        assert_eq!(res, false);
    }
}
