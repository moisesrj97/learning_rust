// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pig_latin(input: String) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let mut result = Vec::new();

    for word in input.split_whitespace() {
        let first_letter = word.chars().nth(0).expect("The input is empty");
        let is_first_letter_vowel = VOWELS.iter().any(|v| v.eq_ignore_ascii_case(&first_letter));

        let mut transformed_word = String::from(word);

        match is_first_letter_vowel {
            false => {
                transformed_word.remove(0);
                transformed_word = transformed_word + "-" + &first_letter.to_string() + "ay";
            }
            true => transformed_word += "-hay",
        }

        result.push(transformed_word)
    }

    return result.join(" ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pig_latin_vowel() {
        let input = String::from("first");

        let response = pig_latin(input);

        let expected_response = String::from("irst-fay");
        assert_eq!(response, expected_response)
    }

    #[test]
    fn pig_latin_consonant() {
        let input = String::from("apple");

        let response = pig_latin(input);

        let expected_response = String::from("apple-hay");
        assert_eq!(response, expected_response)
    }

    #[test]
    fn pig_latin_sentence() {
        let input = String::from("the apple is rotten");

        let response = pig_latin(input);

        let expected_response = String::from("he-tay apple-hay is-hay otten-ray");
        assert_eq!(response, expected_response)
    }
}
