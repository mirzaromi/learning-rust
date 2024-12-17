
fn main() {
    let words = String::from("apple");

    let result = to_pig_latin(&words);

    println!("{result}");
}

fn to_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut chars = word.chars();

    match chars.next() {
        Some(first) if vowels.contains(&first.to_ascii_lowercase()) => {
            format!("{}-hay", word) // Words starting with a vowel
        }
        Some(first) => {
            let rest: String = chars.collect(); // Rest of the word after the first letter
            format!("{}-{}ay", rest, first) // Move the first consonant to the end + "ay"
        }
        None => String::new(), // Handle empty strings
    }
}

// fn to_pig_latin(words: &str) -> String {
//     if words.is_empty() {
//         return String::new();
//     }
//
//     let vowel = vec!['a', 'e', 'i', 'o', 'u'];
//
//     let char = &words.chars().nth(0).unwrap();
//     if vowel.contains(char) {
//         format!("{words}-hay")
//     } else {
//         let first_letter = words.chars().nth(0).unwrap();
//         // slice for 2nd to end words
//         let words_slice = &words[1..];
//
//         format!("{words_slice}-{first_letter}ay")
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pig_latin() {
        assert_eq!(to_pig_latin("first"), "irst-fay"); // Move 'f' to end, add "ay"
        assert_eq!(to_pig_latin("apple"), "apple-hay"); // Word starts with vowel
        assert_eq!(to_pig_latin("banana"), "anana-bay"); // Move 'b' to end
        assert_eq!(to_pig_latin("orange"), "orange-hay"); // Word starts with vowel
        assert_eq!(to_pig_latin(""), ""); // Handle empty string
    }
}