mod stemmer;

use stemmer::Stemmer;

pub fn stem(word: &str) -> Result<String, anyhow::Error> {
    let mut stemmer = Stemmer::new(&word.to_lowercase())?;
    let res = stemmer.stem()?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_stem_word() {
        let res = stem("MULTIDIMENSIONAL").unwrap();

        assert_eq!(res, "multidimension");
    }
}
