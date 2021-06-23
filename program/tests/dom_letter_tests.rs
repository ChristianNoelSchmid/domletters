//! Testing for dominant letters
//!
//! Christian Schmid
//! June 2021

#[cfg(test)]
mod tests {
    use rand::{RngCore, thread_rng};
    use domletters::dom_letters;

    /// The value for ASCII character `a`
    const ASCII_LETTER_MIN: u8 = 97;
    
    /// Tests a series of `100` strings, each of which should have
    /// a `dom_letter` value of `100`.
    #[test]
    fn test_random_strings() {
        for _ in 0..100 {
            let chars = gen_random_string();
            assert_eq!(dom_letters(chars), 100);
        }
    }  

    /// Tests a collection `100` random words
    /// which should have a `dom_letters` value of `10,000`
    #[test]
    fn test_string_paragraph() {
        let mut para = String::new();

        // Generate 100 strings, each with 100 `a`s
        // Append each to para
        for _ in 0..100 {
            let chars = gen_random_string();
            para.push_str(&chars);
        }

        assert_eq!(dom_letters(para), 100 * 100);
    } 

    /// Generates a random string of 100 characters `b-z`. Then
    /// randomly inserts `a` 100 times into the char collection.
    fn gen_random_string() -> String {
        let mut rng = thread_rng();
        let mut chars = String::new();

        // Insert random chars into the string, excluding `a`
        for _ in 0..100 {
            chars.push(((rng.next_u32() % 25) as u8 + ASCII_LETTER_MIN + 1) as char);
        }
        // Insert 100 `a`'s. This will become the dominant character
        for _ in 0..100 {
            chars.insert(rng.next_u32() as usize % chars.len(), 'a');
        }

        chars
    }
}