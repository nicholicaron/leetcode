// Algorithm must be O(n) time 
// Follow up: O(1) extra space complexity (output array does not count as extra space for space complexity analysis) and do not use division operator

// 6ms runtime -- 99.09th percentile

// printf DEBUG iterators using inspect iterator adapter

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<_> = nums
        // Iterates over references to each element in clone of nums
        .iter()
        // Seed scan internal state with 1
        // Replace state with the product of the state and the current value
        // Yield the state (returns an iterator of Option<total_product>)
        .scan(1, |state, num| Some(std::mem::replace(state, *state * num)))
        // collect() consumes the iterator and resolves all Option<T>'s to return a vector of T
        .collect();


    nums
        // Iterate over references to each element in nums
        .iter()
        // reverse the direction of the iterator
        // We want to compare the reverse iteration, with our previous iteration to detect zeros
        .rev()
        // Seed scan internal state with 1
        // Replace state with the product of the state and the current value
        // Yield the state (returns an iterator of Option<total_product>)
        .scan(1, |state, num| Some(std::mem::replace(state, *state * num)))
        // zip() returns an iterator of tuples of the form (Option<T> *from output, taken as mutable reference, then reversed*, Option<T> *from reverse iteration*)
        .zip(output.iter_mut().rev())
        // For each element in the now zipped iterator, replace the value in output with the product of the two Option<T> values
        .for_each(|(x, y)| *y *= x);

    output

}
fn main () {
   // Test cases: 
   assert_eq!(vec![24,12,8,6], product_except_self(vec![1,2,3,4]));
   assert_eq!(vec![0,0,0], product_except_self(vec![0,4,0]));
   assert_eq!(vec![0,0,9,0,0], product_except_self(vec![-1,1,0,-3,3]));
   assert_eq!(vec![0,0,0,0,0], product_except_self(vec![1,0,1,0,1]));
   assert_eq!(vec![0,1], product_except_self(vec![1,0]));
}
