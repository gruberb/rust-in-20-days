use std::io::{self, Write};

fn main() {
    // Create a mutable vector to store our bookmarks
    let mut bookmarks = Vec::new();

    println!("Welcome to Firemarks!");

    loop {
        // Print prompt and flush stdout to ensure it appears before reading input
        print!("Enter a URL (or 'quit' to exit): ");
        io::stdout().flush().unwrap();

        // Create a new String to store user input
        let mut input = String::new();

        // Read a line from stdin into our input String
        io::stdin().read_line(&mut input).unwrap();

        // Remove whitespace and newline characters
        let input = input.trim();

        // Check if user wants to quit
        if input == "quit" {
            break;
        }

        // Add the URL to our bookmarks
        bookmarks.push(input.to_string());
        println!("Bookmark added: {}", input);

        // Display all bookmarks
        println!("\nYour bookmarks:");
        for (index, bookmark) in bookmarks.iter().enumerate() {
            println!("{}. {}", index + 1, bookmark);
        }
    }

    println!("Thank you for using Firemarks!");
}
