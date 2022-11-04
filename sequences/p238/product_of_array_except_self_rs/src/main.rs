// Algorithm must be O(n) time 
// Follow up: O(1) extra space complexity (output array does not count as extra space for space complexity analysis) and do not use division operator

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<_> = nums
        // .iter() transforms the vector into an iterator
        .iter()
        // 1 is the initial value
        // Scan is an iterator adaptor that applies a closure to each element of the iterator and yields the intermediate results.
        // The closure takes two arguments: the first is the previous value of the closure, and the second is the current value of the iterator
        .scan(1, |state, num| Some(std::mem::replace(state, *state * num)))
        // collect() transforms the iterator into back into a vector
        .collect();
    nums
        .iter()
        .rev()
        .scan(1, |state, num| Some(std::mem::replace(state, *state * num)))
        .zip(output.iter_mut().rev())
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
