use std::{env, fs, process, error::Error};


fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Build configuration from command-line arguments or exit on error
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Run the application with the provided configuration
    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}

// Configuration struct to hold search query and file path
struct  Config {
    query: String,
    file_path: String,
}

// Implementation block for the Config struct
impl Config {
    // Builder method to create a Config instance from command-line arguments
    fn build (args: &[String]) -> Result<Config, &'static str> {
        let mut query: String  = Default::default();
        let mut file_path: String = Default::default();

        // Check the number of command-line arguments
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        else if args.len() == 2 {
            query = "".to_string();
            file_path = args[1].clone();
        }
        else if args.len() <= 3 {
            query = args[1].clone();
            file_path = args[2].clone();
        }
    
        // Return a Result with a Config instance or an error message
        Ok(Config { query, file_path })
    }
}

// Run function to execute the application with the given configuration
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the contents of the file specified in the configuration
    let contents = fs::read_to_string(config.file_path)?;

    // Get the number of bytes or characters based on the query
    let bytes_or_chars: usize = get_bytes_or_characters(&contents);

    // Display results based on the query
    if config.query == "-c" {
        println!("Number of bytes in the file: {}", bytes_or_chars);
    }
    else if config.query == "-l" {
        println!("Number of lines in the file: {}", get_number_of_lines(&contents));    
    }
    else if config.query == "-m" {
        println!("Number of characters in the file: {}", bytes_or_chars);
    } 
    else if config.query == "" {
        println!("Number of bytes in the file: {}", bytes_or_chars);
        println!("Number of lines in the file: {}", get_number_of_lines(&contents));    
        println!("Number of characters in the file: {}", bytes_or_chars);
    }
    
    // Return a success Result
    Ok(())
}

// Function to count the number of bytes in a string
fn get_bytes_or_characters(text: &str) -> usize {
    let mut bytes: usize = 0; 
    
    // Iterate over the bytes of the string
    for _byte in text.as_bytes() {
        bytes += 1;
    }
 
    // Return the total count of bytes
    bytes
}

// Function to count the number of lines in a string
fn get_number_of_lines(text: &str) -> usize {
    let mut lines = 0;

    // Iterate over the lines of the string
    for _line in text.lines() {
        lines += 1;
    }

    // Return the total count of lines
    lines
}
