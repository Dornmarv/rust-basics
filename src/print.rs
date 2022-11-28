pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatiing
    println!("{} is from {}", "Marvellous", "Mars");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Marvellous", "Mars", "Code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Marvellous",
        activity = "Football"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 25, 25, 25);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10: {}", 10 + 10);
}
