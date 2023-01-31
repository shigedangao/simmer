mod stemmer;

use stemmer::Stemmer;

/// Run the porter stemmer implementation
///
/// # Arguments
///
/// * `word` - &str
pub fn stem(word: &str) -> Result<String, anyhow::Error> {
    let mut stemmer = Stemmer::new(&word.to_lowercase())?;
    let res = stemmer.stem()?;

    Ok(res)
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

        let correct = vec![
            "fli", "di", "mule", "deni", "di", "agre", "own",
            "humbl", "size", "meet", "state", "siez", "item",
            "sensat", "tradit", "refer", "colon", "plot"
        ];

        let mut stemmed = Vec::new();
        for plural in plurals {
            let res = stem(plural).unwrap();
            stemmed.push(res);
        }

        assert_eq!(stemmed, correct);
    }
}
