pub fn min_operations(boxes: String) -> Vec<i32> {
    // This should be something like boxes.as_bytes().map(|b| b - [ASCII_CONVERSION])
    let bin_arr = boxes
        .as_bytes()
        .iter()
        .map(|c| *c as i32 - 48)
        .collect::<Vec<i32>>();
    let mut output: Vec<i32> = Vec::new();
    for i in 0..bin_arr.len() {
        // we want to walk the length of the array, and calculate some i32 for each run.
        let mut sum: i32 = 0;
        for j in 0..bin_arr.len() {
            if j == i || bin_arr[j] == 0 {
                continue;
            }
            let val = (j as i32 - i as i32).abs();
            sum += val;
        }
        output.push(sum);
    }
    output
}

mod test {
    use super::*;

    #[test]
    pub fn min_operations_test1() {
        assert_eq!(
            min_operations("001011".to_string()),
            vec![11, 8, 5, 4, 3, 4]
        )
    }
}
