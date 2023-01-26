use anyhow::Error;
use super::{
    kind::Kind,
    Stemmer,
};

// constant
const SUFFIX_STEP_1B: [&str; 2] = ["ed", "ing"];
const LSZ: [&str; 3] = ["l", "s", "z"];
const ST: [&str; 2] = ["s", "t"];
const L: [&str; 1] = ["l"];
pub const RULES_TWO: [(&str, &str); 20] = [
    ("ational", "ate"), ("tional", "tion"), ("enci", "ence"), ("anci", "ance"), ("izer", "ize"),
    ("abli", "able"), ("alli", "al"), ("entli", "ent"), ("eli", "e"), ("ousli", "ous"),
    ("ization", "ize"), ("ation", "aze"), ("ator", "ate"), ("alism", "al"), ("iveness", "ive"),
    ("fulness", "ful"), ("ousness", "ous"), ("aliti", "al"), ("iviti", "ive"), ("biliti", "ble")
];
pub const RULES_THREE: [(&str, &str); 7] = [
    ("icate", "ic"), ("ative", ""), ("alize", "al"), ("iciti", "ic"), ("ical", "ic"),
    ("ful", ""), ("ness", "")
];
const RULES_FOUR: [&str; 18] = [
    "al", "ance", "ence", "er", "ic", "able", "ible", "ant", "ement",
    "ment", "ent", "ou", "ism", "ate", "iti", "ous", "ive", "ize"
];

pub(crate) trait PorterStemmerStep1 {
    /// Process step 1a is to remove the plural (s) from a Stemmer
    /// for example a word such as ponies become poni
    fn process_step_one_a(&mut self) -> &mut Self;
    /// Process step 1b is to remove past particles (ed) from a Stemmer
    /// for example a word such as plastered become plaster
    fn process_step_one_b(&mut self) -> Result<&mut Stemmer, Error>;
    /// Process step 1c is to remove any suffix from a word
    /// for example a word such as happy become happi
    fn process_step_one_c(&mut self) -> &mut Stemmer;
}

pub(crate) trait PorterStemmerStep2And3 {
    fn process_step_two_and_three(&mut self, rules: Vec<(&str, &str)>) -> Result<&mut Stemmer, Error>;
}

pub(crate) trait PorterStemmerStep4 {
    fn process_step_four(&mut self) -> Result<&mut Stemmer, Error>;
}

pub(crate) trait PorterStemmerStep5 {
    fn process_step_fifth(&mut self) -> Result<String, Error>;
}

impl PorterStemmerStep1 for Stemmer {
    // Step 1a
    fn process_step_one_a(&mut self) -> &mut Self {
        let word = match &self.word {
            w if w.ends_with("sses") => w.trim_end_matches("es").to_string(),
            w if w.ends_with("ies") => w.trim_end_matches("es").to_string(),
            w if w.ends_with("ss") => w.to_string(),
            w if w.ends_with("s") => w.trim_end_matches("s").to_string(),
            _ => self.word.to_owned()
        };

        self.word = word;

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
                let trimmed = self.word.trim_end_matches(suffix);
                // check if the trimmed word is a vowel
                if Kind::has_vowel(trimmed) {
                    // process the intermediary externally
                    self.word = process_intermediary_step_b(trimmed)?;

                    return Ok(self);
                }
            }
        }

        Ok(self)
    }

    // Step 1c
    fn process_step_one_c(&mut self) -> &mut Self {
        if Kind::has_vowel(&self.word) && self.word.ends_with("y") {
            self.word = format!("{}i", self.word.trim_end_matches("y"))
        }

        self
    }
}

impl PorterStemmerStep2And3 for Stemmer {
    // Step 2
    fn process_step_two_and_three(&mut self, rules: Vec<(&str, &str)>) -> Result<&mut Stemmer, Error> {
        // Recompute the porter Stemmer in order to get a measure
        self.recompute_porter_stemmer()?;

        if self.get_measure() > 0 {
            rules
                .iter()
                .for_each(|(rule, replacement)| {
                    if self.word.ends_with(rule) {
                        self.word = self.word.replace(rule, replacement);
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
            RULES_FOUR.iter()
                .for_each(|rule| {
                    if self.word.ends_with(rule) {
                        self.word = self.word.replace(rule, "");
                    }
                });

            // Special case of *S or *T and finish by ion
            if self.word.ends_with("ion") {
                if Stemmer::check_end_letter(&self.word, ST.to_vec()) {
                    self.word = self.word.replace("ion", "");
                }
            }
        }

        Ok(self)
    }
}

impl PorterStemmerStep5 for Stemmer {
    fn process_step_fifth(&mut self) -> Result<String, Error> {
        // recompute the porter Stemmer
        self.recompute_porter_stemmer()?;

        // Step 5a
        if self.get_measure() > 1 && self.word.ends_with("e") {
            self.word = self.word.trim_end_matches("e").to_string();

            return Ok(self.word.to_owned());
        }

        if self.get_measure() == 1 && !self.check_cvc_pattern() && self.word.ends_with("e") {
            self.word = self.word.trim_end_matches("e").to_string();

            return Ok(self.word.to_owned());
        }

        // Step 5b
        if self.get_measure() > 1 &&
            Kind::end_with_double_consonent(&self.word) &&
            Stemmer::check_end_letter(&self.word, L.to_vec()) {
                self.word.pop();
        }

        Ok(self.word.to_string())
    }
}

/// In the case if the following step of 1b is true:
/// (*v*) ED
/// (*v*) ING
/// then we need to do an additional process which will generalize the word
///
/// # Arguments
///
/// * `trimmed_word` - &str
fn process_intermediary_step_b(trimmed_word: &str) -> Result<String, Error> {
    // Case where the trimmed_word ended with
    // - AT
    // - BL
    // - IZ
    if trimmed_word.ends_with("at") || trimmed_word.ends_with("bl") || trimmed_word.ends_with("iz") {
        return Ok(format!("{}e", trimmed_word));
    }

    // Case where the trimmed_word end with a double consonent & is not an L, S or Z
    // we remove the last consonent
    if Kind::end_with_double_consonent(trimmed_word) &&
        !Stemmer::check_end_letter(trimmed_word, LSZ.into()) {
            let exploded: Vec<char> = trimmed_word.chars()
                .enumerate()
                .filter_map(|(idx, c)| {
                    if idx < trimmed_word.len() - 1 {
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect();

            return Ok(exploded.iter().collect());
    }

    // last check (m=1 and *o) -> E
    let stemmer = Stemmer::new(trimmed_word)?;
    if stemmer.get_measure() == 1 && stemmer.check_cvc_pattern() {
        return Ok(format!("{}e", trimmed_word));
    }

    Ok(trimmed_word.to_string())
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
            .process_step_two_and_three(RULES_TWO.to_vec())
            .unwrap()
            .process_step_two_and_three(RULES_THREE.to_vec())
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
            .process_step_two_and_three(RULES_TWO.to_vec())
            .unwrap()
            .process_step_two_and_three(RULES_THREE.to_vec())
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
            .process_step_two_and_three(RULES_TWO.to_vec())
            .unwrap()
            .process_step_two_and_three(RULES_THREE.to_vec())
            .unwrap()
            .process_step_four()
            .unwrap()
            .process_step_fifth()
            .unwrap();

        assert_eq!(processed, "character");
    }
}
