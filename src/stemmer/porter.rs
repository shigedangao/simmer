use crate::error::SimmerError;
use super::kind::Kind;

#[derive(Debug, PartialEq, Clone)]
pub enum ParsedWord {
    C(Vec<char>),
    V(Vec<char>),
    None
}

impl ParsedWord {
    /// Build a ParsedWord from a list of char
    ///
    /// # Arguments
    ///
    /// * `char_list` - Vec<char>
    fn build(char_list: Vec<char>) -> ParsedWord {
        match char_list.last() {
            Some(c) => {
                match Kind::from(*c) {
                    Kind::Consonent => ParsedWord::C(char_list),
                    Kind::Vowel => ParsedWord::V(char_list),
                    Kind::None => ParsedWord::None
                }
            },
            None => ParsedWord::None
        }
    }

    /// Get a vetor of kinds for each characters which set whether it's a vowel or consonent
    ///
    /// # Arguments
    ///
    /// * `word` - &str
    pub fn parse(word: &str) -> Result<Vec<ParsedWord>, SimmerError> {
        let mut kinds = Vec::new();
        // split the string into single char
        let characters: Vec<char> = word.chars().collect();
        // temporary list
        let mut current_chars_list: Vec<char> = Vec::new();

        for chars_idx in 0..characters.len() {
            let Some(current_char) = characters.get(chars_idx) else {
                return Err(SimmerError::Character)
            };

            // get the last item of the current list
            match current_chars_list.last() {
                Some(item) => {
                    let curr_char_kind = Kind::from(*current_char);
                    // insert a character if it has the same type in order to associated a set of character with consonent or vowel
                    // if match the same type, insert the character to the temporary character vector
                    if Kind::from(*item) == curr_char_kind {
                        current_chars_list.push(current_char.to_owned());
                    } else {
                        kinds.push(ParsedWord::build(current_chars_list.to_owned()));
                        // flush the temporary list
                        current_chars_list.clear();
                        // pushing the character which does not match the previous kind
                        current_chars_list.push(*current_char);
                    }
                },
                None => current_chars_list.push(*current_char)
            }

            // push the kinds with the remaining list
            if chars_idx == characters.len() - 1 {
                kinds.push(ParsedWord::build(current_chars_list.clone()));
            }
        }

        Ok(kinds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_get_kind_vec() {
        let word = "toy";
        let list = ParsedWord::parse(word).unwrap();

        assert_eq!(*list.get(0).unwrap(), ParsedWord::C(vec!['t']));
        assert_eq!(*list.get(1).unwrap(), ParsedWord::V(vec!['o', 'y']));
    }

    #[test]
    fn expect_to_get_kind_complex_word() {
        let word = "trouble";
        let list = ParsedWord::parse(word).unwrap();

        assert_eq!(*list.get(0).unwrap(), ParsedWord::C(vec!['t', 'r']));
        assert_eq!(*list.get(1).unwrap(), ParsedWord::V(vec!['o', 'u']));
        assert_eq!(*list.get(2).unwrap(), ParsedWord::C(vec!['b', 'l']));
        assert_eq!(*list.get(3).unwrap(), ParsedWord::V(vec!['e']));
    }
}
