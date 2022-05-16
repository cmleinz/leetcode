pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut output = String::new();
    let mut i = 0;
    loop {
        let target_char = match strs[0].chars().nth(i) {
            Some(c) => c,
            None => return output,
        };
        if strs.iter().all(|a| a.chars().nth(i) == Some(target_char)) {
            output.push(target_char);
            i += 1;    
        } else {
            return output;
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn longest_common_prefix_test1() {
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(), 
                "racecar".to_string(), 
                "car".to_string()]), 
            "".to_string())
    }

    #[test]
    pub fn longest_common_prefix_test2() {
         assert_eq!(
             longest_common_prefix(vec![
                "flower".to_string(), 
                "flow".to_string(), 
                "flight".to_string()]), 
            "fl".to_string())
    } 
}