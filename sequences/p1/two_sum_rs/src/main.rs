// Takes an array of integers and an integer target value as arguments
// Returns the unique pair of integers in the array which sum to the target value  

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();
    let mut soln = Vec::new();
    for i in 0..nums.len() {
        let diff: i32 = target - nums[i];
        if let Some(j) = seen.get(&diff) {
            soln.push(i.try_into().unwrap());
            soln.push(*j);
        } else {
            seen.insert(nums[i], i.try_into().unwrap());
        }
    }
    soln
}


fn main() {
    let nums: Vec<i32> = vec![2, 5, 4, 9, 32, 12, 88, 23, 54, 21, 1, 30];
    println!("{:?}", two_sum(nums, 100));
    println!("Passed inspection, sir!");
}
