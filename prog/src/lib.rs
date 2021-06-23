use std::ops::Index;

use regex::Regex;

/// The value for ASCII character `a`
const ASCII_LETTER_MIN: usize = 97;

/// Finds the most dominant letter in a series of words,
/// excluding any words which have non-ascii letters (a-z).
/// ## Example
/// ```
/// use prog::dom_letters;
/// let count = dom_letters("'Night, night!' said the knight to the knight one night.".to_string());
/// assert_eq!(count, 5); // There are 5 't's
/// ```
pub fn dom_letters(words: String) -> u32 {
    // A count value for each possible ASCII char
    let mut letter_counts: [u32; 26];
    let mut running_count = 0;

    // The regular expression which will filter each word
    let re = Regex::new("^[a-z]+$").unwrap();

    let words = words.to_lowercase();
    let words = words.split_ascii_whitespace().filter(|w| re.is_match(*w));
    for word in words.clone() {
        println!("{}", word);
    }

    for word in words {
        letter_counts = [0; 26];
        for c in word.chars() {
            letter_counts[(c as usize - ASCII_LETTER_MIN)] += 1;
        }
        running_count += letter_counts.iter().max().unwrap();
    }
    
    running_count
}
