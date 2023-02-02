use super::{
    porter::ParsedWord,
    kind::Kind
};

/// Compute the number of combination of VC (Vowel -> Consonent) in the CVCV model
///
/// # Arguments
///
/// * `parsed_words` - &Vec<ParsedWord>
pub fn compute_measures(parsed_words: &Vec<ParsedWord>) -> i32 {
    let mut previous = Kind::None;
    let mut measured = 0;

    for pw in parsed_words {
        // Set the previous value to the first value initially and skip the checking
        // as we're unable to count
        if previous == Kind::None {
            previous = Kind::from(pw);

            continue;
        }

        let current = Kind::from(pw);
        // check whether we have the combination 'VC (Vowel -> Consonent'
        if previous == Kind::Vowel && current == Kind::Consonent {
            measured += 1;
        }

        previous = current;
    }

    measured
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_measure_zero_combination() {
        let pw = vec![
            ParsedWord::C(vec!['t', 'r']),
            ParsedWord::V(vec!['e', 'e'])
        ];

        let m = compute_measures(&pw);

        assert_eq!(m, 0);
    }

    #[test]
    fn expect_to_measure_one_combination() {
        let pw = vec![
            ParsedWord::C(vec!['t', 'r']),
            ParsedWord::V(vec!['o', 'u']),
            ParsedWord::C(vec!['b', 'l']),
            ParsedWord::V(vec!['e'])
        ];

        let m = compute_measures(&pw);

        assert_eq!(m, 1);
    }

    #[test]
    fn expect_to_measure_two_combination() {
        let pw = vec![
            ParsedWord::C(vec!['t', 'r']),
            ParsedWord::V(vec!['o', 'u']),
            ParsedWord::C(vec!['b', 'l']),
            ParsedWord::V(vec!['e']),
            ParsedWord::C(vec!['s'])
        ];

        let m = compute_measures(&pw);

        assert_eq!(m, 2);
    }
}
