use crate::passes::convert_ffi_types::TypeReplacer;
use crate::passes::replace_raw_pointers::RawPointerSanitizer;
use syn::{parse_file, Error, File};

/// A monadic type wrapping a [syn::File] abstract syntax tree (AST) whose monadic
/// functions are passes that mutate the monad's held AST value.
///
/// (See https://en.wikipedia.org/wiki/Monad_(functional_programming) for more
/// background on monads.)
pub struct MonadicAst {
    pub ast: File,
}

impl MonadicAst {
    /// Returns a monadic value wrapping the constructed AST of the given Rust code.
    pub fn new(file_content: &str) -> Result<Self, Error> {
        let ast = parse_file(file_content)?;
        Ok(Self { ast })
    }

    /// Returns the monad's held AST value, consuming the monad.
    pub fn ast(self) -> File {
        self.ast
    }

    /// Returns a formatted string representation of the monad's held AST.
    pub fn result(&self) -> String {
        prettyplease::unparse(&self.ast)
    }

    /// Replaces C foreign function interface (FFI) types in the AST with their Rust
    /// equivalents, e.g. libc::c_int -> i32.
    pub fn convert_ffi_types(self) -> Self {
        TypeReplacer::new().bind(self)
    }

    /// Identifies declared raw pointers and replaces them with their safe Rust type
    /// equivalent determined via static analysis on their access patterns or usages.
    pub fn replace_raw_pointers(self) -> Self {
        RawPointerSanitizer::default().bind(self)
    }
}

impl From<File> for MonadicAst {
    /// Analogous to the conventional `return :: a -> M a` unit operation.
    ///
    /// Receives a `syn::File` AST and wraps it into a monadic value, where `M`  is
    /// a `MonadicAst` and `a` is a `syn::File` abstract syntax trees.
    fn from(ast: File) -> Self {
        Self { ast }
    }
}

/// Analogous to the conventional `bind :: (M a) -> (a -> M b) -> (M b)`
/// (or `>>=`) operation.
///
/// Receives a monadic AST wrapper `M a` and returns the result `M b` of applying the
/// `bind()` method on the unwrapped AST `a`, where `M` is a `MonadicAst` and `a`,`b`
/// are `syn::File` abstract syntax trees.
pub trait Pass {
    fn bind(&mut self, monad: MonadicAst) -> MonadicAst;
}
