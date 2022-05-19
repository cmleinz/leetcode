// pub fn is_valid(s: String) -> bool {
//     if s.len() % 2 == 1 {
//         false
//     } else {
//         for (i, c) in s.char_indices() {
//             s.chars().position(|a| )
//         }
//         s.char_indices().find_map(|(i, c)| )
//         true
//     }
// }

// mod test {
//     use super::*;

//     #[test]
//     fn is_valid_test1() {
//         assert!(is_valid("()".to_string()))
//     }

//     #[test]
//     fn is_valid_test2() {
//         assert!(!is_valid("(]".to_string()))
//     }

//     #[test]
//     fn is_valid_test3() {
//         assert!(is_valid("()[]{}".to_string()))
//     }

//     #[test]
//     fn is_valid_test4() {
//         assert!(is_valid("([]{})".to_string()))
//     }
// }