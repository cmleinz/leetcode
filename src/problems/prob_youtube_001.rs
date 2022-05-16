use std::collections::HashMap;

/// Returns the list of words which can be created from the provided phone number
/// 
/// # Arguments
/// 
/// * `number` - a phone number consisting of only numberics
/// * `words` - a list of words  which will be searched over to determine if they can be contained within the phone number
/// 
/// # Exmplanation
/// 
/// A complete explanation of the problem is available at think [url](https://www.youtube.com/watch?v=PIeiiceWe_w)
/// This solution is my own and is not nearly as well constructed as the one provided in the video, but it does meet the requirements
pub fn phone_substrings(number: String, words: Vec<String>) -> Vec<String> {
    let mut output = Vec::new();
    let mut dial_map = HashMap::new();
    dial_map.insert('2', vec!['a', 'b', 'c']);
    dial_map.insert('3', vec!['d', 'e', 'f']);
    dial_map.insert('4', vec!['g', 'h', 'i']);
    dial_map.insert('5', vec!['j', 'k', 'l']);
    dial_map.insert('6', vec!['m', 'n', 'o']);
    dial_map.insert('7', vec!['p', 'q', 'r', 's']);
    dial_map.insert('8', vec!['t', 'u', 'v']);
    dial_map.insert('9', vec!['w', 'x', 'y', 'z']);

    for word in words {
        let mut chars = word.chars();
        let mut char_nth = chars.next().unwrap();
        for n_char in number.chars() {
            if n_char == '1' || n_char == '0' {
                continue;
            }
            let stuff = dial_map.get(&n_char).unwrap();
            if stuff.contains(&char_nth) {
                char_nth = match chars.next() {
                    Some(c) => c,
                    None => {
                        output.push(word);
                        break;
                    },
                };
            } else {
                chars = word.chars();
                char_nth = chars.next().unwrap();
            }
        }
    }
    output
}

mod test {
    use super::*;

    #[test]
    fn phone_substrings_test1() {
        assert_eq!(
            phone_substrings("3662277".to_string(), vec!["foo".to_string(), "bar".to_string(), "baz".to_string(), "foobar".to_string(), "emo".to_string(), "cap".to_string(), "car".to_string(), "cat".to_string()]),
            vec!["foo".to_string(), "bar".to_string(), "foobar".to_string(), "emo".to_string(), "cap".to_string(), "car".to_string()]
        )
    }

    #[test]
    fn phone_substrings_test2() {
        assert_eq!(
            phone_substrings("2536368".to_string(), vec!["clement".to_string(), "clemdot".to_string(), "apple".to_string(), "foobar".to_string()]),
            vec!["clement".to_string(), "clemdot".to_string()]
        )
    }
}