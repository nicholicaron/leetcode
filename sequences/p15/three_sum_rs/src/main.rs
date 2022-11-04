// This is relatively slow (25th percentile), and uses considerable memory. Eventually optimize 
use std::collections::HashSet;

fn main() {
    let nums: Vec<i32> = Vec::from([-1, 0, 1, 2, -1, -4]);
    let res = three_sum(nums);

    println!("{:?}", res);
} 

// Given an array of integers, return all unique triplet combinations that sum to zero
// It seems the iterative approach may be the best way to go here
// We can use a hash map to store the values we've seen
fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // Use a hashset here, since we don't care about the keys, we can let rust decide
    let mut seen: HashSet<Vec<i32>> = HashSet::new();
    // resultant vector, specified capacity for memory efficiency
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(nums.len()/3 + 1);

    nums.sort();
    // iterate through sorted array, only have to check elements less than 0
    // For each element after our current index, check if the sum of the two is equal to the negative of the current element
    // If so, add the triplet to the result vector, then push triplet to the hashset to check if
    // it's been seen before
    for (index, &num) in nums.iter().enumerate() {
        if num > 0 {
            break;
        }
        // Then we can pinch the next and outer elements
        let mut left_index = index + 1;
        let mut right_index = nums.len() - 1;
        while left_index < right_index {
            let sum = num + nums[left_index] + nums[right_index];
            if sum == 0 {
                let triplet: Vec<i32> = vec![num, nums[left_index], nums[right_index]];
                // Triplet is sorted (avoids duplicates), use the hashset to check if it's been seen before
                // values are i32 so it's cheap to clone
                if seen.insert(triplet.clone()) {
                    // We have a new triplet, add it to output vec 
                    result.push(triplet);
                }
                // Increment left index, decrement right index
                left_index += 1;
                right_index -= 1;
            } else if sum < 0 {
                // Sum is too small, increment left index
                left_index += 1;
            } else {
                // Sum is too large, decrement right index
                right_index -= 1;
            }
        }

    }
    return result;

}
