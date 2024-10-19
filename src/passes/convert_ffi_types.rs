use crate::monad::ast::{MonadicAst, Pass};
use quote::ToTokens;
use std::collections::HashMap;
use syn::{visit_mut::VisitMut, Type};

static RULES: &[(&str, &str)] = &[
    ("libc::c_int", "i32"),
    ("libc::c_uint", "u32"),
    ("libc::c_void", "()"),
    ("libc::c_char", "i8"),
    ("libc::c_uchar", "u8"),
    ("libc::c_long", "i64"),
    ("libc::c_ulong", "u64"),
];

pub struct TypeReplacer {
    rules: HashMap<&'static str, &'static str>,
}

impl TypeReplacer {
    pub fn new() -> Self {
        Self {
            rules: RULES.iter().cloned().collect(),
        }
    }
}

impl VisitMut for TypeReplacer {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if let Type::Path(type_path) = ty {
            let type_string = type_path.to_token_stream().to_string().replace(" ", "");
            if let Some(to) = self.rules.get(type_string.as_str()) {
                if let Ok(new_type) = syn::parse_str::<Type>(to) {
                    *ty = new_type;
                    return;
                }
            }
        }
        syn::visit_mut::visit_type_mut(self, ty);
    }
}

impl Pass for TypeReplacer {
    fn bind(&mut self, mut monad: MonadicAst) -> MonadicAst {
        self.visit_file_mut(&mut monad.ast);
        monad
    }
}
