use anyhow::Error;

const CONSONENT_LIST: [char; 20] = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'];
const VOWEL_LIST: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Kind {
    Consonent,
    Vowel
}

#[derive(Debug)]
pub(crate) enum ParsedWord {
    Consonents(Vec<char>),
    Vowels(Vec<char>),
    None
}

impl ParsedWord {
    fn build(char_list: Vec<char>) -> ParsedWord {
        let last_char = char_list.last();
        if last_char.is_none() {
            return ParsedWord::None;
        }

        match Kind::from(last_char.unwrap()) {
            Kind::Consonent => ParsedWord::Consonents(char_list),
            Kind::Vowel => ParsedWord::Vowels(char_list)
        }
    }

    /// Get a vetor of kinds for each characters which set whether it's a vowel or consonent
    /// 
    /// # Arguments
    /// 
    /// * `word` - &str
    fn get_kinds_for_word(word: &str) -> Result<Vec<ParsedWord>, Error> {
        let mut kinds = Vec::new();
        // split the string into single char
        let characters:Vec<char> = word.chars().collect();
        // temporary list
        let mut current_chars_list: Vec<char> = Vec::new();

        for chars_idx in 0..characters.len() {
            let Some(current_char) = characters.get(chars_idx) else {
                return Err(Error::msg("Unable to get the current character"))
            };

            // get the type of the current character
            let curr_char_type = Kind::from(current_char);

            // get the last item of the current list
            match current_chars_list.last() {
                Some(item) => {
                    // insert a character if it has the same type in order to associated a set of character with consonent or vowel
                    let item_char_type = Kind::from(item);
                    // if match the same type, insert the character to the temporary character vector
                    if item_char_type == curr_char_type {
                        current_chars_list.push(current_char.to_owned());
                    } else {
                        kinds.push(ParsedWord::build(current_chars_list.clone()));
                        // flush the temporary list
                        current_chars_list.clear();
                        // pushing the character which does not match the previous kind
                        current_chars_list.push(current_char.to_owned());
                    }
                },
                None => current_chars_list.push(current_char.to_owned())
            }

            // push the kinds with the remaining list
            if chars_idx == characters.len() - 1 {
                kinds.push(ParsedWord::build(current_chars_list.clone()));
            }
        }

        Ok(kinds)
    }
}

impl From<&char> for Kind {
    fn from(c: &char) -> Self {
        if CONSONENT_LIST.contains(&c) {
            return Kind::Consonent;
        }

        Kind::Vowel
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn expect_to_get_kind_vec() {
        let word = "eye";

        let list = ParsedWord::get_kinds_for_word(word);

        dbg!(list);
    }

    #[test]
    fn expect_to_get_kind_complex_word() {
        let word = "trouble";

        let list = ParsedWord::get_kinds_for_word(word);

        dbg!(list);
    }
}