mod stemmer;
mod util;
pub mod error;

use error::SimmerError;
use stemmer::Stemmer;
use util::AsciiUtil;

/// Get the stem from a word
///
/// # Arguments
///
/// * `word` - &str
pub fn stem(word: &str) -> Result<String, SimmerError> {
    let mut stemmer = Stemmer::new(&word.to_lowercase())?;
    let res = stemmer.stem()?;

    Ok(res)
}

/// Stem a sentence by splitting the sentence by whitespace
/// If the sentence contains ascii punctuation the word will be skipped
///
/// # Arguments
///
/// * `sentence` - &str
pub fn stem_sentence(sentence: &str) -> Result<Vec<String>, SimmerError> {
    // split the sentence by spaces and remove words which has a punctuation
    let words: Vec<String> = sentence.split_whitespace()
        .map(|w| w.remove_ascii_punctuation())
        .collect();

    let mut stemmed = Vec::new();
    // call the stemmer for each words
    for word in words {
        let mut stemmer = Stemmer::new(&word.to_lowercase())?;
        let stem = stemmer.stem()?;

        stemmed.push(stem);
    }

    Ok(stemmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_stem_plurals_words() {
        let multidimensional = stem("MULTIDIMENSIONAL").unwrap();
        let detestable = stem("detestable").unwrap();
        let caresses = stem("caresses").unwrap();

        assert_eq!(multidimensional, "multidimension");
        assert_eq!(detestable, "detest");
        assert_eq!(caresses, "caress");
    }

    #[test]
    fn expect_to_stem_multiple_words() {
        let plurals = vec![
            "flies", "dies", "mules", "denied", "died", "agreed", "owned",
            "humbled", "sized", "meeting", "stating", "siezing", "itemization",
            "sensational", "traditional", "reference", "colonizer", "plotted"
        ];

        let corrects = vec![
            "fli", "di", "mule", "deni", "di", "agre", "own",
            "humbl", "size", "meet", "state", "siez", "item",
            "sensat", "tradit", "refer", "colon", "plot"
        ];

        let mut stemmed = Vec::new();
        for plural in plurals {
            let res = stem(plural).unwrap();
            stemmed.push(res);
        }

        assert_eq!(stemmed, corrects);
    }

    #[test]
    fn expect_to_stem_sentence() {
        let sentence = "His eyes were dancing with humor.";

        let stems = super::stem_sentence(sentence).unwrap();
        let stem_sentence = stems.join(" ");

        assert_eq!(stem_sentence, "hi eye were danc with humor")
    }
}
