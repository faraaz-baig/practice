pub fn run() {
    // Print to Console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Faraaz", "Bangalore");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Faraaz", "Bangalore", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {game}",
        name = "Faraaz",
        game = "League Of Legends"
    );
}
