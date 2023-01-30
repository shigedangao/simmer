use anyhow::Error;
use super::{
    kind::Kind,
    Stemmer,
};

// Constant
const SUFFIX_STEP_1B: [&str; 2] = ["ed", "ing"];
const END_LETTERS_LSZ: [&str; 3] = ["l", "s", "z"];
const END_LETTERS_ST: [&str; 2] = ["s", "t"];
const END_LETTERS_L: [&str; 1] = ["l"];
pub const RULES_TWO_SUFFIX: [(&str, &str); 20] = [
    ("ational", "ate"), ("tional", "tion"), ("enci", "ence"), ("anci", "ance"), ("izer", "ize"),
    ("abli", "able"), ("alli", "al"), ("entli", "ent"), ("eli", "e"), ("ousli", "ous"),
    ("ization", "ize"), ("ation", "aze"), ("ator", "ate"), ("alism", "al"), ("iveness", "ive"),
    ("fulness", "ful"), ("ousness", "ous"), ("aliti", "al"), ("iviti", "ive"), ("biliti", "ble")
];
pub const RULES_THREE_SUFFIX: [(&str, &str); 7] = [
    ("icate", "ic"), ("ative", ""), ("alize", "al"), ("iciti", "ic"), ("ical", "ic"),
    ("ful", ""), ("ness", "")
];
const RULES_FOUR_SUFFIX: [&str; 18] = [
    "al", "ance", "ence", "er", "ic", "able", "ible", "ant", "ement",
    "ment", "ent", "ou", "ism", "ate", "iti", "ous", "ive", "ize"
];

pub(crate) trait PorterStemmerStep1 {
    /// Process step 1a is to remove the plural (s) from a Stemmer
    /// for example a word such as ponies become poni
    fn process_step_one_a(&mut self) -> &mut Self;
    /// Process step 1b is to remove past particles (ed) from a Stemmer
    /// for example a word such as plastered become plaster
    fn process_step_one_b(&mut self) -> Result<&mut Self, Error>;
    /// In the case if the following step of 1b is true:
    /// (*v*) ED
    /// (*v*) ING
    /// then we need to do an additional process which will generalize the word
    ///
    /// # Arguments
    ///
    /// * `trimmed_word` - &str
    fn process_step_one_b_intermediary(&mut self, trimmed: &str) -> Result<&mut Self, Error>;
    /// Process step 1c is to remove any suffix from a word
    /// for example a word such as happy become happi
    fn process_step_one_c(&mut self) -> &mut Self;
}

pub(crate) trait PorterStemmerStep2And3 {
    /// Step 2 and 3 replace the suffix by checking each rules on the targeted word if only M > 0
    ///
    /// # Arguments
    ///
    /// * `rules` - &[(&str, &str)]
    fn process_step_two_and_three(&mut self, rules: &[(&str, &str)]) -> Result<&mut Stemmer, Error>;
}

pub(crate) trait PorterStemmerStep4 {
    /// Step 4 replace the suffix with a set of rules if M > 1
    fn process_step_four(&mut self) -> Result<&mut Stemmer, Error>;
}

pub(crate) trait PorterStemmerStep5 {
    /// Step 5 replace the E suffix if M > 1 or M = 1 depending on the detail of subrules
    fn process_step_fifth(&mut self) -> Result<String, Error>;
}

impl PorterStemmerStep1 for Stemmer {
    // Step 1a
    fn process_step_one_a(&mut self) -> &mut Self {
        let word = match &self.word {
            w if w.ends_with("sses") => w.trim_end_matches("es"),
            w if w.ends_with("ies") => w.trim_end_matches("es"),
            w if w.ends_with("ss") => w,
            w if w.ends_with('s') => w.trim_end_matches('s'),
            _ => &self.word
        };

        self.word = word.to_string();

        self
    }

    // Step 1b
    fn process_step_one_b(&mut self) -> Result<&mut Stemmer, Error> {
        // expect to return a word ending with 'ee' instead of 'eed'
        // this handle the case of (m>0) EED -> EE
        if self.word.ends_with("eed") {
            // make a copy of the existing word as the word might not have M > 0
            let original = self.word.clone();

            // trim the word
            let mut trimmed = self.word.to_owned();
            trimmed = trimmed.trim_end_matches("eed").to_string();
            // recompute the Stemmer for the trimmed word
            self.parse_new_word(&trimmed)?;

            if self.get_measure() > 0 {
                // feed -> feed
                // agreed -> agree
                // in this case we can only trim the d this will return the 'ee'
                self.word.push_str("ee");
                return Ok(self)
            } else {
                self.word = original;
                return Ok(self)
            }
        }

        // This handle the case of:
        // (*v*) ED
        // (*v*) ING
        for suffix in SUFFIX_STEP_1B {
            if self.word.ends_with(suffix) {
                // trim the end
                let trimmed = self.word.trim_end_matches(suffix).to_owned();
                // check if the trimmed word is a vowel
                if Kind::has_vowel(&trimmed) {
                    // process the intermediary externally
                    self.process_step_one_b_intermediary(&trimmed)?;

                    return Ok(self);
                }
            }
        }

        Ok(self)
    }

    fn process_step_one_b_intermediary(&mut self, trimmed: &str) -> Result<&mut Self, Error> {
        // Case where the trimmed_word ended with
        // - AT
        // - BL
        // - IZ
        if trimmed.ends_with("at") || trimmed.ends_with("bl") || trimmed.ends_with("iz") {
            self.word = format!("{trimmed}e");
            return Ok(self);
        }

        // Case where the trimmed_word end with a double consonent & is not an L, S or Z
        // we remove the last consonent
        if Kind::end_with_double_consonent(trimmed) &&
        !Stemmer::check_end_letter(trimmed, &END_LETTERS_LSZ) {
            let exploded: Vec<char> = trimmed.chars()
                .enumerate()
                .filter_map(|(idx, c)| {
                    if idx < trimmed.len() - 1 {
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect();

            self.word = exploded.iter().collect();

            return Ok(self);
        }

        // last check (m=1 and *o) -> E
        self.parse_new_word(trimmed)?;
        if self.get_measure() == 1 && self.check_cvc_pattern() {
            self.word = format!("{trimmed}e");

            return Ok(self);
        }

        Ok(self)
    }

    // Step 1c
    fn process_step_one_c(&mut self) -> &mut Self {
        if Kind::has_vowel(&self.word) && self.word.ends_with('y') {
            self.word = format!("{}i", self.word.trim_end_matches('y'))
        }

        self
    }
}

impl PorterStemmerStep2And3 for Stemmer {
    // Step 2
    fn process_step_two_and_three(&mut self, rules: &[(&str, &str)]) -> Result<&mut Stemmer, Error> {
        // Recompute the porter Stemmer in order to get a measure
        self.recompute_porter_stemmer()?;

        if self.get_measure() > 0 {
            rules
                .iter()
                .for_each(|(rule, replacement)| {
                    if self.word.ends_with(rule) {
                        let mut word = self.word.trim_end_matches(rule).to_string();
                        word.push_str(replacement);

                        self.word = word;
                    }
                });
        }

        Ok(self)
    }
}

impl PorterStemmerStep4 for Stemmer {
    fn process_step_four(&mut self) -> Result<&mut Stemmer, Error> {
        self.recompute_porter_stemmer()?;

        if self.get_measure() > 1 {
            RULES_FOUR_SUFFIX.iter()
                .for_each(|rule| {
                    if self.word.ends_with(rule) {
                        self.word = self.word.trim_end_matches(rule).to_string();
                    }
                });

            // Special case of *S or *T and finish by ion
            if self.word.ends_with("ion") && Stemmer::check_end_letter(&self.word, &END_LETTERS_ST) {
                self.word = self.word.trim_end_matches("ion").to_string();
            }
        }

        Ok(self)
    }
}

impl PorterStemmerStep5 for Stemmer {
    fn process_step_fifth(&mut self) -> Result<String, Error> {
        let original = self.word.to_string();

        // Step 5a
        if self.word.ends_with('e') {
            self.word.pop();
            self.recompute_porter_stemmer()?;

            if self.get_measure() > 1 {
                return Ok(self.word.to_owned());
            } else if self.get_measure() == 1 && !self.check_cvc_pattern() {
                return Ok(self.word.to_owned());
            } else {
                self.word = original;
            }
        }

        // Step 5b
        if self.get_measure() > 1 &&
            Kind::end_with_double_consonent(&self.word) &&
            Stemmer::check_end_letter(&self.word, &END_LETTERS_L) {
                self.word.pop();
        }

        Ok(self.word.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_respect_all_step_especially_a() {
        let mut word = Stemmer::new("caresses").unwrap();
        let mut second_word = Stemmer::new("ponies").unwrap();

        let processed = word
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        let second_processed = second_word
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        assert_eq!(processed.word, "caress");
        assert_eq!(second_processed.word, "poni");
    }

    #[test]
    fn expect_to_respect_all_step_especially_b_one() {
        let mut feed = Stemmer::new("feed").unwrap();
        let mut agreed = Stemmer::new("agreed").unwrap();

        let processed_feed = feed
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        let processed_agreed = agreed
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        assert_eq!(processed_feed.word, "feed");
        assert_eq!(processed_agreed.word, "agree");
    }

    #[test]
    fn expect_to_respect_all_step_especially_b_two() {
        let mut plastered = Stemmer::new("plastered").unwrap();
        let mut bled = Stemmer::new("bled").unwrap();

        let processed_plastered = plastered
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        let processed_bled = bled
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        assert_eq!(processed_plastered.word, "plaster");
        assert_eq!(processed_bled.word, "bled");
    }

    #[test]
    fn expect_to_respect_all_step_especially_b_two_intermediary() {
        let mut conflated = Stemmer::new("conflated").unwrap();
        let mut hopping = Stemmer::new("hopping").unwrap();
        let mut falling = Stemmer::new("falling").unwrap();

        let processed_conflated = conflated
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        let processed_hopping = hopping
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        let processed_falling = falling
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        assert_eq!(processed_conflated.word, "conflate");
        assert_eq!(processed_hopping.word, "hop");
        assert_eq!(processed_falling.word, "fall");
    }

    #[test]
    fn expect_to_respect_all_step_especially_c() {
        let mut word = Stemmer::new("happy").unwrap();

        let processed = word
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        assert_eq!(processed.word, "happi");
    }

    #[test]
    fn expect_to_respect_rules_two() {
        let mut word = Stemmer::new("decisiveness").unwrap();

        let processed = word
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c()
            .process_step_two_and_three(&RULES_TWO_SUFFIX)
            .unwrap()
            .process_step_two_and_three(&RULES_THREE_SUFFIX)
            .unwrap();

        assert_eq!(processed.word, "decisive");
    }

    #[test]
    fn expect_to_respect_rules_four() {
        let mut word = Stemmer::new("allowance").unwrap();

        let processed = word
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c()
            .process_step_two_and_three(&RULES_TWO_SUFFIX)
            .unwrap()
            .process_step_two_and_three(&RULES_THREE_SUFFIX)
            .unwrap()
            .process_step_four()
            .unwrap();

            assert_eq!(processed.word, "allow");
    }

    #[test]
    fn expect_to_respect_rules_fifth() {
        let mut word = Stemmer::new("characterization").unwrap();

        let processed = word
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c()
            .process_step_two_and_three(&RULES_TWO_SUFFIX)
            .unwrap()
            .process_step_two_and_three(&RULES_THREE_SUFFIX)
            .unwrap()
            .process_step_four()
            .unwrap()
            .process_step_fifth()
            .unwrap();

        assert_eq!(processed, "character");
    }

    #[test]
    fn expect_to_respect_every_rules() {
        let mut word = Stemmer::new("meeting").unwrap();

        let processed = word
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c()
            .process_step_two_and_three(&RULES_TWO_SUFFIX)
            .unwrap()
            .process_step_two_and_three(&RULES_THREE_SUFFIX)
            .unwrap()
            .process_step_four()
            .unwrap()
            .process_step_fifth()
            .unwrap();

        assert_eq!(processed, "meet");
    }
}
