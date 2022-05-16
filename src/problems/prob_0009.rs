pub fn is_palindrome(x: i32) -> bool {
    let x_string = x.to_string();
    x_string == x_string.chars().rev().collect::<String>() 
}

mod test {
    use super::*;

    #[test]
    fn is_palindrome_test1() {
        assert!(is_palindrome(121))
    }

    #[test]
    fn is_palindrome_test2() {
        assert!(!is_palindrome(10))
    }

    #[test]
    fn is_palindrome_test3() {
        assert!(!is_palindrome(-121))
    }
}