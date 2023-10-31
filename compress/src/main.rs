extern crate flate2; // This line specifies an external dependency on the flate2 crate, which provides Gzip compression and decompression functionality

use flate2::write::GzEncoder;
use flate2::Compression;
// These lines import specific components from the flate2 crate. GzEncoder is used for Gzip compression, and Compression allows you to specify compression settings
use std::env::args;
// This line imports the args function from the std::env module, which allows you to access command-line arguments.
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
// These lines import file-related components from the standard library. File is used for file I/O, 
// copy is for copying data between sources and sinks, and BufReader is used to efficiently read from a file.
use std::time::Instant;
// This line imports the Instant struct from the standard library, which is used for measuring time.

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`"); // If the number of arguments is not 3, this line prints a usage message to the standard error stream
        return; // This line exits the program early if the argument count is not 3
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // This line opens the input file specified as the second command-line argument, 
    // wraps it in a BufReader for efficient reading, and stores it in the input variable.
    let output = File::create(args().nth(2).unwrap()).unwrap();
    // This line creates the output file specified as the third command-line argument and stores it in the output variable.
    let mut encoder = GzEncoder::new(output, Compression::default());
    // This line initializes a Gzip encoder (GzEncoder) that wraps the output file. It uses the default compression settings.
    let start = Instant::now();
    // This line starts a timer to measure the time taken for the compression process.
    copy(&mut input, &mut encoder).unwrap();
    // This line copies data from the input (source) to the Gzip encoder (encoder). This effectively compresses the input data. 
    // The unwrap method is used to handle any potential errors by panicking if they occur.
    let output = encoder.finish().unwrap();
    // This line finalizes the compressed output by calling the finish method on the Gzip encoder. The result is stored in the output variable.
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    // This line prints the length (size) of the source (input) file using the metadata method to retrieve file metadata.
    println!("Target len: {:?}", output.metadata().unwrap().len());
    // This line prints the length (size) of the target (compressed) file using the metadata method on the finalized output.
    println!("Elapsed: {:?}", start.elapsed());
    // This line prints the elapsed time taken for the compression process using the Instant timer.
}
