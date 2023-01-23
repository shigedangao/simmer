mod kind;
mod measure;
mod porter;

use anyhow::Error;
use self::kind::Kind;
use self::porter::ParsedWord;

// Constant
const AVOID_CONSONENTS: [char; 3] = ['w', 'x', 'y'];

pub(crate) struct Stem<'a> {
    word: &'a str,
    porter_stemmer: Vec<ParsedWord>
}

impl<'a> Stem<'a> {
    /// Create a new Stem struct and build the porter_stemmer representation of the word
    /// 
    /// # Arguments
    /// 
    /// * `word` - &'a str
    pub fn new(word: &'a str) -> Result<Stem<'a>, Error>{
        let porter_stemmer = ParsedWord::parse(word)?;

        Ok(Stem {
            word,
            porter_stemmer
        })
    }

    /// Check the end of a word (either if it's a S, L, T...) (*S)
    /// 
    /// # Arguments
    /// 
    /// * `letters` - Vec<&str>
    fn check_end_letter(&self, letters: Vec<&str>) -> bool {
        for letter in letters {
            if self.word.ends_with(letter) {
                return true;
            }
        }

        false
    }

    /// Check if the word has a vowel (*v*)
    fn has_vowel(&self) -> bool {
        Kind::has_vowel(self.word)
    }

    /// Check if the word end with a double consonent (any consonent) (*d)
    fn end_with_double_consonent(&self) -> bool {
        Kind::end_with_double_consonent(self.word)
    }

    /// Check the chain of Consonent -> Vowel -> Consonent pattern (*o)
    /// /!\ Note that the second consonent must not be W, X or Y
    fn check_cvc_pattern(&self) -> bool {
        if self.porter_stemmer.len() < 3 {
            return false;
        }

        // get the last 3 items of the porter stemmer
        let end_length = self.porter_stemmer.len() - 3;
        let items = self.porter_stemmer.get(end_length..);
        if items.is_none() {
            return false;
        }

        let mut valid = false;
        for (idx, item) in items.unwrap().into_iter().enumerate() {
            // @TODO we can use a reference later...
            let kind = Kind::from(item.clone());

            if idx == 0 && kind == Kind::Consonent {
                valid = true;
            } else if idx == 1 && kind == Kind::Vowel {
                valid = true;
            } else if idx == 2 {
                // get the inner value whether it's not W,X,Y
                let character = item.as_char();
                if !AVOID_CONSONENTS.contains(&character) && kind == Kind::Consonent {
                    valid = true;
                } else {
                    valid = false;
                }
            } else {
                valid = false;
            }
        }

        valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_end_cvc_pattern() {
        let word = "rapperswil";
        let stemmer = Stem::new(word).unwrap();
        
        let is_cvc = stemmer.check_cvc_pattern();

        assert_eq!(is_cvc, true);
    }

    #[test]
    fn expect_to_not_end_cvc_pattern() {
        let word = "hello";
        let stemmer = Stem::new(word).unwrap();
        
        let is_cvc = stemmer.check_cvc_pattern();

        assert_eq!(is_cvc, false);
    }

    #[test]
    fn expect_to_not_end_cvc_consonent_pattern() {
        let word = "nywow";
        let stemmer = Stem::new(word).unwrap();
        
        let is_cvc = stemmer.check_cvc_pattern();

        assert_eq!(is_cvc, false);
    }
}