use quote::ToTokens;
use syn::{visit_mut::VisitMut, File, Type};
use crate::monad::ast::{MonadicAst, Pass};

static RULES: &[(&str, &str)] = &[
    ("libc::c_int", "i32"),
    ("libc::c_uint", "u32"),
    ("libc::c_void", "()"),
    ("libc::c_char", "i8"),
    ("libc::c_uchar", "u8"),
    ("libc::c_long", "i64"),
    ("libc::c_ulong", "u64"),
];

pub struct TypeReplacer<'a> {
    rules: &'a [(&'a str, &'a str)],
}

impl TypeReplacer<'_> {
    pub fn new() -> Self {
        Self { rules: RULES }
    }
}

impl VisitMut for TypeReplacer<'_> {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if let Type::Path(type_path) = ty {
            let type_string = type_path.to_token_stream().to_string().replace(" ", "");

            for (from, to) in self.rules {
                if type_string == *from {
                    if let Ok(new_type) = syn::parse_str::<Type>(to) {
                        *ty = new_type;
                        return;
                    }
                }
            }
        }
        syn::visit_mut::visit_type_mut(self, ty);
    }
}

impl Pass for TypeReplacer<'_> {
    fn bind(&mut self, mut ast: File) -> MonadicAst {
        self.visit_file_mut(&mut ast);
        MonadicAst::from(ast)
    }
}
