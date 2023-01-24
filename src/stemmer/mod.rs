mod kind;
mod measure;
mod porter;
mod steps;

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
    fn check_end_letter(word: &str, letters: Vec<&str>) -> bool {
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
                    false
                } else {
                    let mut valid = false;
                    // fused options
                    let consonents = kinds.get(0)
                        .zip(kinds.get(2));

                    if let Some((c_one, c_two)) = consonents {
                        if *c_one == Kind::Consonent && *c_two == Kind::Consonent {
                            valid = true;
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
                }
            },
            None => false
        }
    }

    /// Get the measure from the porter stemmer
    fn get_measure(&self) -> i32 {
        return measure::compute_measures(&self.porter_stemmer)
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
