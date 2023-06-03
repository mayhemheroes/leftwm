use honggfuzz::fuzz;
use leftwm::config::absolute_path;
use std::path::PathBuf;

fn main() {
    // Define the fuzzing loop
    loop {
        // Fuzzing input will be provided by Honggfuzz
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(path) = std::str::from_utf8(data) {
                // Call the absolute_path function with the fuzzed input
                let _ = absolute_path(path);
                
            }
        });
    }
}
