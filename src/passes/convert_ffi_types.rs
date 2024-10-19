use quote::{quote, ToTokens};
use std::fmt::Pointer;
use std::fs;
use syn::visit::{self, Visit};
use syn::{parse_file, visit_mut::VisitMut, FnArg, ItemFn, PatType, Type, TypePtr};

static RULES: &[(&str, &str)] = &[
    ("libc::c_int", "i32"),
    ("libc::c_uint", "u32"),
    ("libc::c_void", "()"),
    ("libc::c_char", "i8"),
    ("libc::c_uchar", "u8"),
    ("libc::c_long", "i64"),
    ("libc::c_ulong", "u64"),
];

struct TypeReplacer<'a> {
    rules: &'a [(&'a str, &'a str)],
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

pub fn replace_types_in_file(
    file_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut ast = parse_file(&content)?;
    let mut replacer = TypeReplacer { rules: RULES };

    replacer.visit_file_mut(&mut ast);

    Ok(prettyplease::unparse(&ast))
}
