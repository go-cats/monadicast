use syn::{parse_file, Error, File};

pub struct MonadicAst {
    ast: File
}

impl MonadicAst {
    pub fn new(file_content: String) -> Result<Self, Error> {
        let mut ast = parse_file(&file_content)?;
        Ok(Self { ast })
    }
}