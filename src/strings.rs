pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push string
    hello.push_str("Faraaz");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check is empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains: {}", hello.contains("Faraaz"));

    // Replace
    println!("Replace: {}", hello.replace("Faraaz", "Trick2g"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);
}
