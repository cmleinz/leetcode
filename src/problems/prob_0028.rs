pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle == "" {
        return 0;
    }
    let needle = needle.as_bytes();
    let haystack = haystack.as_bytes();
    if haystack.len() < needle.len() {
        return -1;
    }
    for i in 0..(haystack.len() - needle.len() + 1) {
        if haystack[i] == needle[0] {
            if &haystack[i..(i + needle.len())] == needle {
                return i as i32;
            }
        }
    }
    -1
}

mod test {
    use super::*;

    #[test]
    fn str_str_test1() {
        assert_eq!(str_str("hello".into(), "ll".into()), 2)
    }

    #[test]
    fn str_str_test2() {
        assert_eq!(str_str("aaaaa".into(), "bba".into()), -1)
    }

}