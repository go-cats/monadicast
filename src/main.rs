use std::fs;
use crate::type_converter::replace_types_in_file;

mod type_converter;

fn main() {
    match replace_types_in_file("./example/input.rs") {
        Ok(new_content) => {
            fs::write("./example/output.rs", new_content).expect("Unable to write file");
            println!("Successfully converted types and wrote to output.rs");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
