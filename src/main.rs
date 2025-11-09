use std::env;
use std::fs;
use std::process;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a filename was provided
    if args.len() < 2 {
        eprintln!("Error: No filename provided.");
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    // Read the file content
    let content = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // Process each line and collect the cleaned lines
    let cleaned_lines: Vec<String> = content
        .lines()
        .map(|line| {
            // Check for a line comment marker "//"
            if let Some(comment_start_index) = line.find("//") {
                // Get the part of the string that is the comment
                let comment_part = &line[comment_start_index..];

                // Check if the comment contains the "!!" emoji
                if comment_part.contains("‼️") {
                    // If it does, truncate the line just before the comment,
                    // and trim any trailing whitespace.
                    return line[..comment_start_index].trim_end().to_string();
                } else {
                    // If it's a comment but not one of mine, keep the line as is
                    return line.to_string();
                }
            } else {
                // If there's no comment, keep the line as is
                return line.to_string();
            }
        })
        .collect();

    // ‼️ Join the cleaned lines back together with newlines
    let output = cleaned_lines.join("\n");

    // ‼️ Overwrite the original file with the cleaned content
    match fs::write(filename, output) {
        Ok(_) => {
            // ‼️ Print success message to stderr
            eprintln!("Successfully cleaned and overwrote '{}'.", filename);
        }
        Err(e) => {
            eprintln!("Error writing back to file '{}': {}", filename, e);
            process::exit(1);
        }
    }
}
