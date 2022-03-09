// This isn't very fast, or very good on memory, but I think it reads much nicer 
// than more efficient solutions. Improving efficiency would require a more imperative solution, 
// removing iterator adaptors

// takes a string, breaks it into characters, casts the chars to u32 values (in case of unicode)
// then collects them into a vector. This vector is then sorted to compare to the other
// parameter
fn is_anagram(s: String, t: String) -> bool {
    let mut s_vec: Vec<u32> = s.chars().map(|letter| letter as u32).collect();
    s_vec.sort();

    let mut t_vec: Vec<u32> = t.chars().map(|letter| letter as u32).collect();
    t_vec.sort();

    return s_vec == t_vec
    
}

fn main() {
    let s: String = "rustyferris".to_string();
    let t: String = "sirtusyrefr".to_string();
    println!("{}", is_anagram(s, t));
}


