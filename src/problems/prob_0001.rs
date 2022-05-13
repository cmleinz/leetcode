pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, num) in nums.iter().enumerate() {
        match nums.iter().skip(i+1).position(|&q| q == target - num ) {
            Some(j) => return vec![i as i32, j as i32 + i as i32 + 1],
            None => continue,
        }
    }
    panic!("Couldn't find a solution");
}

mod test {
    use super::*;

    #[test]
    fn two_sum_test1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }
    #[test]
    fn two_sum_test2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }
    #[test]
    fn two_sum_test3() {
        assert_eq!(two_sum(vec![3,3], 6), vec![0, 1])
    }
}