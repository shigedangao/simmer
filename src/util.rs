pub trait AsciiUtil {
    /// Remove ascii punctuation from a word
    fn remove_ascii_punctuation(&self) -> String;
}

impl AsciiUtil for str {
    fn remove_ascii_punctuation(&self) -> String {
        let chars: Vec<char> = self.chars().collect();
        let filtered_word: String = chars.into_iter()
            .filter(|c| !c.is_ascii_punctuation())
            .collect();

        filtered_word
    }
}
