fn main() {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut first_index: Option<i32> = None;
        let mut second_index: Option<i32> = None;
        let mut first_value: Option<i32> = None;
        let mut second_value: Option<i32> = None;

        for (i, x) in nums.iter().enumerate() {
            if first_value == None {
                first_value = Some(*x);
            }
            for (i, y) in nums.iter().enumerate() {
                if y + Some(first_value) == target {
                    second_index = *y;
                    break;
                }
            }
        }

        let my_vec: Vec<i32> = vec![first_index, second_index];
        my_vec
    }

    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9))
}
