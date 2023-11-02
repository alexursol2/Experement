// Import the `Error` trait from the `std` module, which is used for error handling.
use std::error::Error;

// Import the `csv` library, which provides CSV file parsing functionality.
use csv;

// Define a function `read_from_file` that takes a path to a CSV file as a parameter.
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Create a CSV reader from the provided file path, and handle any potential errors.
    let mut reader = csv::Reader::from_path(path)?;

    // Iterate over the CSV records in the reader.
    for result in reader.records() {
        // Unwrap the result, which might contain a record or an error, and assign it to `record`.
        let record = result?;

        // Print the record to the console. In a real application, you might process the record differently.
        println!("{:?}", record);
    }

    // Return a `Result` indicating a successful operation with no return value.
    Ok(())
}

// Define the `main` function, which is the entry point of the program.
fn main() {
    // Attempt to call the `read_from_file` function with the path to a CSV file.
    if let Err(e) = read_from_file("./customers.csv") {
        // If an error occurs during the execution of `read_from_file`, print the error message to the standard error stream.
        eprintln!("{}", e);
    }
}
