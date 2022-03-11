// Note to self: sort_unstable() results in considerable speedups, so when dealing with vectors of primitive types,
// prefer unstable sort over stable sort [thanks clippy]

// takes a string, breaks it into characters, casts the chars to u32 values (in case of unicode)
// then collects them into a vector. This vector is then sorted to compare to the other
// parameter
fn is_anagram(s: String, t: String) -> bool {
    let mut s_vec: Vec<u32> = s.chars().map(|letter| letter as u32).collect();
    s_vec.sort_unstable();

    let mut t_vec: Vec<u32> = t.chars().map(|letter| letter as u32).collect();
    t_vec.sort_unstable();

    s_vec == t_vec
    
}

fn main() {
    let s: String = "rustyferris".to_string();
    let t: String = "sirtusyrefr".to_string();
    println!("{}", is_anagram(s, t));
}


