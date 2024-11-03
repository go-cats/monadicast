use monadicast::MonadicAst;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("./example/input.rs")?;
    let output = MonadicAst::new(&content)?
        .convert_ffi_types()
        .replace_raw_pointers()
        // .replace_while_loop()
        .result();

    fs::write("./example/output.rs", output)?;
    println!("Successfully converted types and wrote to output.rs");
    Ok(())
}
