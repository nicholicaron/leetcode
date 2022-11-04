use std::collections::HashSet;
// Currently exceeds time limit for large input

fn main() {
    //
    // Is a nested loop the best way to go? 
    let nums: Vec<i32> = Vec::from([0,0,0,0]);
    let res = three_sum(nums);

    println!("{:?}", res);
} 

// Given an array of integers, return all unique triplet combinations that sum to zero
// It seems the iterative approach may be the best way to go here
// We can use a hash map to store the values we've seen
fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // Use a hashset here, since we don't care about the keys, we can let rust decide
    let mut seen: HashSet<Vec<i32>> = HashSet::new();
    let mut result: Vec<Vec<i32>> = Vec::new();

    // By changing the bounds for each loop, we can ensure that each element i, j, k is unique
    // I think this also reduces the number of unnecessary iterations
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            for k in j + 1..nums.len() {
                if nums[i] + nums[j] + nums[k] == 0 {
                    // We have a match
                    // We need to check if we've seen this triplet before
                    // We can do this by sorting the triplet and storing it in a hash map
                    let mut triplet: Vec<i32> = Vec::new();
                    triplet.push(nums[i]);
                    triplet.push(nums[j]);
                    triplet.push(nums[k]);
                    // Sort the triplet to avoid duplicates
                    triplet.sort();

                    // Returns true if the value was not present, false if it was
                    if seen.insert(triplet.clone()) {
                        // We have a new triplet
                        // Add it to the output
                        result.push(triplet);
                    }
                }
            }
        }
    }
    return result;

}
