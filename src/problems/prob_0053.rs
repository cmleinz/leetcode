pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums.last().unwrap();
    let mut current_sum = 0;
    for num in &nums {
        current_sum = std::cmp::max(num.copy(), current_sum + num);
        max_sum = std::cmp::max(max_sum, &current_sum);
    }
    *max_sum
}
