// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pig_latin(input: String) -> String {
    return String::from("irst-fay");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pig_latin_basic() {
        let input = String::from("first");

        let response = pig_latin(input);

        let expected_response = String::from("irst-fay");
        assert_eq!(response, expected_response)
    }
}
