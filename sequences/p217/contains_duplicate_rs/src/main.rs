use std::collections::HashMap;
// use std::collections::HashSet;

/* Given an integer array nums, return true if any value appears at least
   twice in the array and return false if every element is distinct.

   10ms runtime 93rd percentile */
fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen: HashMap<i32, bool> = HashMap::new();
        
        for num in nums {
            if let Some(_i) = seen.get(&num){
                return true;
            } else {
                seen.insert(num, true);
            }
        }
        false
}

/*  This impl uses ~16% less memory, but is 110% slower than prev. solution, hmmm?  
    HashSet generally slower than HashMap, HashMap more suitable to compiler 
    optimizations
    fn contains_duplicate(nums: Vec<i32>) -> bool {
      let mut seen = HashSet::new();
      for num in nums {
          if !seen.insert(num) {return true;}
      }
      false
  } */


fn main() {
    let nums: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", contains_duplicate(nums));
}
