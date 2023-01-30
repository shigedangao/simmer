mod kind;
mod measure;
mod porter;
mod steps;

use anyhow::Error;
use self::kind::Kind;
use self::porter::ParsedWord;
use self::steps::{
    PorterStemmerStep1,
    PorterStemmerStep2And3,
    PorterStemmerStep4,
    PorterStemmerStep5,
    RULES_TWO_SUFFIX,
    RULES_THREE_SUFFIX
};

// Constant
const AVOID_CONSONENTS: [char; 3] = ['w', 'x', 'y'];

#[derive(Debug)]
pub struct Stemmer {
    word: String,
    porter_stemmer: Vec<ParsedWord>
}

impl Stemmer {
    /// Create a new Stem struct and build the porter_stemmer representation of the word
    ///
    /// # Arguments
    ///
    /// * `word` - &'a str
    pub fn new(word: &str) -> Result<Stemmer, Error>{
        let porter_stemmer = ParsedWord::parse(word)?;

        Ok(Stemmer {
            word: word.to_string(),
            porter_stemmer
        })
    }

    /// Check the end of a word (either if it's a S, L, T...) (*S)
    ///
    /// # Arguments
    ///
    /// * `letters` - &[&str]
    fn check_end_letter(word: &str, letters: &[&str]) -> bool {
        for letter in letters {
            if word.ends_with(letter) {
                return true;
            }
        }

        false
    }

    /// Check the chain of Consonent -> Vowel -> Consonent pattern (*o)
    /// /!\ Note that the second consonent must not be W, X or Y
    fn check_cvc_pattern(&self) -> bool {
        if self.word.len() < 3 {
            return false;
        }

        let end = self.word.get(self.word.len() - 3..);
        match end {
            Some(v) => {
                // split the char
                let kinds: Vec<Kind> = v.chars()
                    .enumerate()
                    .filter_map(|(idx, c)| {
                        if idx == 2 && AVOID_CONSONENTS.contains(&c) {
                            return None;
                        }

                        Some(Kind::from(c))
                    })
                    .collect();

                if kinds.len() < 3 {
                    return false
                }

                let mut valid = false;
                // fused options
                let consonents = kinds.get(0)
                    .zip(kinds.get(2));

                if let Some((c_one, c_two)) = consonents {
                    if *c_one == Kind::Consonent && *c_two == Kind::Consonent {
                        valid = true;
                    } else {
                        // return false in that case there's no need to further check as the assumed consonent are not consonent
                        return false;
                    }
                }

                if let Some(vowel) = kinds.get(1) {
                    if *vowel == Kind::Vowel {
                        valid = true;
                    } else {
                        valid = false;
                    }
                }

                valid
            },
            None => false
        }
    }

    /// Get the measure from the porter stemmer
    fn get_measure(&self) -> i32 {
        measure::compute_measures(&self.porter_stemmer)
    }

    /// Parse a new word and set the stemmer to this new word
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Self
    /// * `word` - &str
    fn parse_new_word(&mut self, word: &str) -> Result<(), Error> {
        self.porter_stemmer = ParsedWord::parse(word)?;
        self.word = word.to_string();

        Ok(())
    }

    /// recompute the porter stemmer for the current word
    fn recompute_porter_stemmer(&mut self) -> Result<(), Error> {
        self.porter_stemmer = ParsedWord::parse(&self.word)?;

        Ok(())
    }


    pub fn stem(&mut self) -> Result<String, Error> {
        let result = self
            .process_step_one_a()
            .process_step_one_b()?
            .process_step_one_c()
            .process_step_two_and_three(&RULES_TWO_SUFFIX)?
            .process_step_two_and_three(&RULES_THREE_SUFFIX)?
            .process_step_four()?
            .process_step_fifth()?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_end_cvc_pattern() {
        let word = "rapperswil";
        let stemmer = Stemmer::new(word).unwrap();

        let is_cvc = stemmer.check_cvc_pattern();

        assert_eq!(is_cvc, true);
    }

    #[test]
    fn expect_other_to_not_end_cvc_pattern() {
        let word = "meet";
        let stemmer = Stemmer::new(word).unwrap();

        let is_cvc = stemmer.check_cvc_pattern();

        assert_eq!(is_cvc, false);
    }


    #[test]
    fn expect_to_not_end_cvc_pattern() {
        let word = "hello";
        let stemmer = Stemmer::new(word).unwrap();

        let is_cvc = stemmer.check_cvc_pattern();

        assert_eq!(is_cvc, false);
    }

    #[test]
    fn expect_to_not_end_cvc_consonent_pattern() {
        let word = "nywow";
        let stemmer = Stemmer::new(word).unwrap();

        let is_cvc = stemmer.check_cvc_pattern();

        assert_eq!(is_cvc, false);
    }
}
