// 100th percentile runtime, 75.31th percentile memory usage
//
//
// let's make a stack on top of a vec (as opposed to a linked list) because vectors are laid out
// contiguously in memory, so traversal is efficient because they better lend themselves to caching
// Insertion and deletion in the back of the collection takes amortized constant time because the CPU 
// does not have to recall the row address selection demultiplexer as frequently (i think lol).

fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for char in s.chars() {
        match char {
            // if an opening parentheses variant is found, push the corresponding closing variant
            // to the stack
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            // If a closing parentheses variant is found, pop an element off the stack. The new top
            // element on the stack should equal the current (closing parentheses variant) element.
            // If it doesn't, the string is invalid
            '}' | ')' | ']' if Some(char) != stack.pop() => return false,
            _ => (),
        }
    }
    // if there are still elements on the stack, there are more opening than closing parentheses,
    // and hence the string is invalid
    stack.is_empty()
}

fn main() {
    let test1 = "()[]{}".to_string(); // true
    let test2 = "(}".to_string(); // false
    println!(
        "Test case 1: {}\nTest case 2: {}",
        is_valid(test1),
        is_valid(test2)
    );
}
