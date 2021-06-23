//! Library providing method which tests for dominant word count
//! In a collection of ASCII alpha values.
//!
//! Christian Schmid
//! June 2021

use regex::Regex;

/// The value for ASCII character `a`
const ASCII_LETTER_MIN: usize = 97;

/// Finds the most dominant letter in a series of words,
/// excluding any words which have non-ascii letters (only a-z, A-Z)
/// ## Example
/// ```
/// use domletters::dom_letters;
///
/// let mut sentence = String::from("'Night, night!' said the knight ");
/// sentence.push_str("to the knight one night.");
///
/// let count = dom_letters(sentence);
/// assert_eq!(count, 7); // Each alpha word has a DWC of 1. 7 total.
/// ```
pub fn dom_letters(words: String) -> u32 {
    // A count value for each possible ASCII char
    let mut letter_counts: [u32; 26];
    let mut running_count = 0;

    // The regular expression which will filter each word
    let re = Regex::new("^[a-z]+$").unwrap();

    // Convert the letters to lowercase, split them by whitespace, and
    // test if they match the regular expression
    let words = words.to_lowercase();
    let words = words.split_ascii_whitespace().filter(|w| re.is_match(*w));

    // For each word, count each letter, and return the count of the letter 
    // which shows up most often (or the count of the ones tied for most).
    for word in words {
        letter_counts = [0; 26];
        for c in word.chars() {
            letter_counts[(c as usize - ASCII_LETTER_MIN)] += 1;
        }
        running_count += letter_counts.iter().max().unwrap();
    }
    
    running_count
}
