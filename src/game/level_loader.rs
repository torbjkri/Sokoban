use std::end;
use std::fs;

pub fn load_level() {
    let contents = fs::read_to_string("assets/tutorial.level")
        .expect("Should have been able to read the file.");

    let lines = contents.lines();

    // Iterate over lines
    // Create board element type vectors
    // parse stuff
    // Return board state
}
