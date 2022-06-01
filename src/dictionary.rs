/// This module describes the main storage trait for the application - a Dictionary.
///
/// Important to mention: the main language of the dictionary is English,
/// so all of the data pieces included into the dictionary can be converted through
/// the main (canonical) language - English.
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// A Dictionary record representation.
/// Consists of two main strings:
/// - native - UTF8 string in native language
/// - english - translation in the English language
///
/// Categories and examples can be used to further categorize
/// the word or to supply some additional metadata with it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Word {
    pub native: String,
    pub english: String,
    pub categories: Vec<String>,
    pub examples: Vec<String>,
}

impl Word {
    pub fn new(
        native: String,
        english: String,
        categories: Vec<String>,
        examples: Vec<String>,
    ) -> Self {
        Word {
            native,
            english,
            categories,
            examples,
        }
    }
}

/// The main data storage trait to be used in for dictionary applications.
pub trait Dictionary {
    /// Defines the 2-char code of the language for which this dictionary is
    /// used. For example: "RU" - Russian, "EN" - English, "TR" - Turkish;
    const LANG: &'static str;

    /// Get the number of words in the dictionary.
    fn count(&self) -> usize;

    /// Adds a new word to the dictionary;
    fn add_word(
        &mut self,
        word: String,
        en_word: String,
        categories: Vec<String>,
        examples: Vec<String>,
    ) -> Result<()>;

    /// Get a word (and the word metadata - such as translation and categories).
    fn get_word(&self, word: &String) -> Option<&Word>;

    /// Get a word by its English translation.
    fn get_word_en(&self, en_word: &String) -> Option<&Word>;

    /// Get all the words as a Vector;
    fn get_all(&self) -> &Vec<Word>;
}
