use syn::{parse_file, Error, File};
use crate::passes::convert_ffi_types::TypeReplacer;

pub struct MonadicAst {
    pub ast: File
}

impl MonadicAst {
    pub fn from_content(file_content: &str) -> Result<Self, Error> {
        let ast = parse_file(file_content)?;
        Ok( Self { ast } )
    }

    pub fn ast(self) -> File {
        self.ast
    }

    pub fn result(&self) -> String {
        prettyplease::unparse(&self.ast)
    }

    pub fn convert_ffi_types(self) -> Self {
        TypeReplacer::new().bind(self.ast)
    }
}

impl From<File> for MonadicAst {
    fn from(ast: File) -> Self {
        Self { ast }
    }
}

pub trait Pass {
    fn bind(&mut self, ast: File) -> MonadicAst;
}