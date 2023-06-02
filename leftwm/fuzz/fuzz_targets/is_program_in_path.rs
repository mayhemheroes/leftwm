use honggfuzz::fuzz;
use leftwm::config::is_program_in_path;

fn main() {
    // Define the fuzzing loop
    loop {
        // Fuzzing input will be provided by Honggfuzz
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(program) = std::str::from_utf8(data) {
                // Call the is_program_in_path function with the fuzzed input
                let _ = is_program_in_path(program);

            }
        });
    }
}