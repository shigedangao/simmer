use anyhow::Error;
use super::{
    kind::Kind,
    Stem,
};

// constant
const SUFFIX_STEP_1B: [&str; 2] = ["ed", "ing"];
const LSZ: [&str; 3] = ["l", "s", "z"];

// trait
pub(crate) trait PorterStemmerSteps: where Self: Sized {
    /// Step 1.a
    fn process_step_one_a(self) -> Self;
    /// Step 1.b
    fn process_step_one_b(self) -> Result<Self, Error>;
    /// Step 1.c
    fn process_step_one_c(self) -> Self;
}

impl PorterStemmerSteps for String {
    fn process_step_one_a(self) -> String {
        let word = match self {
            w if w.ends_with("sses") => w.trim_end_matches("es").to_string(),
            w if w.ends_with("ies") => w.trim_end_matches("es").to_string(),
            w if w.ends_with("ss") => w,
            w if w.ends_with("s") => w.trim_end_matches("s").to_string(),
            _ => self
        };

        word
    }

    fn process_step_one_b(self) -> Result<Self, Error> {
        // expect to return a word ending with 'ee' instead of 'eed'
        if self.ends_with("eed") {
            let trimmed = self.trim_end_matches("eed");
            let stemmer = Stem::new(trimmed)?;

            if stemmer.get_measure() > 0 {
                // feed -> feed
                // agreed -> agree
                // in this case we can only trim the d this will return the 'ee'
                return Ok(self.trim_end_matches("d").to_string())
            } else {
                return Ok(self)
            }
        }

        for suffix in SUFFIX_STEP_1B {
            if self.ends_with(suffix) {
                // trim the end
                let trimmed = self.trim_end_matches(suffix);
                // check if the trimmed word is a vowel
                if Kind::has_vowel(trimmed) {
                    // process the intermediary externally
                    let value = process_intermediary_step_b(trimmed)?;

                    return Ok(value);
                }
            }
        }

        Ok(self)
    }

    fn process_step_one_c(self) -> Self {
        if Kind::has_vowel(&self) && self.ends_with("y") {
            return format!("{}i", self.trim_end_matches("y"))
        }

        self
    }
}

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
        !Stem::check_end_letter(trimmed_word, LSZ.into()) {
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

    let stemmer = Stem::new(trimmed_word)?;
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
        let word = "caresses".to_string();
        let second_word = "ponies".to_string();

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

        assert_eq!(processed, "caress");
        assert_eq!(second_processed, "poni");
    }

    #[test]
    fn expect_to_respect_all_step_especially_b_one() {
        let feed = "feed".to_string();
        let agreed = "agreed".to_string();

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

        assert_eq!(processed_feed, "feed");
        assert_eq!(processed_agreed, "agree");
    }

    #[test]
    fn expect_to_respect_all_step_especially_b_two() {
        let plastered = "plastered".to_string();
        let bled = "bled".to_string();

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

        assert_eq!(processed_plastered, "plaster");
        assert_eq!(processed_bled, "bled");
    }

    #[test]
    fn expect_to_respect_all_step_especially_b_two_intermediary() {
        let conflated = "conflated".to_string();
        let hopping = "hopping".to_string();
        let falling = "falling".to_string();

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

        assert_eq!(processed_conflated, "conflate");
        assert_eq!(processed_hopping, "hop");
        assert_eq!(processed_falling, "fall");
    }

    #[test]
    fn expect_to_respect_all_step_especially_c() {
        let word = "happy".to_string();

        let processed = word
            .process_step_one_a()
            .process_step_one_b()
            .unwrap()
            .process_step_one_c();

        assert_eq!(processed, "happi");
    }
}
