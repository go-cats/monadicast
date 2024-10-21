//! See https://immunant.com/blog/2023/03/lifting/ for more information on
//! pointer derivation graph (PDG) matching.

use crate::monad::ast::Pass;
use crate::MonadicAst;
use quote::quote;
use std::collections::{BTreeMap, HashMap};
use syn::visit::Visit;
use syn::{FnArg, Ident, PatIdent, PatType, Type, TypePtr};

/// Represents a permission that a raw pointer *p will need at the point in
/// the program p is defined and used.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PointerAccess {
    Write,     // The program writes to the pointee.
    Unique,    // The pointer is the only way to access the given memory location.
    Free,      // The pointer will eventually be passed to free.
    OffsetAdd, // We'll add an offset to the pointer, e.g. array element access.
    OffsetSub, // We'll subtract an offset to the pointer.
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum RustPointerType {
    ImmutableReference, // &T
    MutableReference,   // &mut T
    CellReference,      // &Cell<T>
    UniquePointer,      // Box<T>
    ImmutableSlice,     // &[T]
    MutableSlice,       // &mut [T]
    UniqueSlicePointer, // Box<[T]>
    Undefined,          // ...for unsupported combinations
}

static ACCESSES: &[PointerAccess] = &[
    PointerAccess::Write,
    PointerAccess::Unique,
    PointerAccess::Free,
    PointerAccess::OffsetAdd,
    PointerAccess::OffsetSub,
];

impl PointerAccess {
    /// Returns the Rust safe pointer type corresponding to the given pointer access
    /// permissions, if any exists, and RustPointerType::Undefined otherwise.
    ///
    /// The permissions to type mapping is determined by the following table:
    /// Write - Unique - Free - Offset  |  Resulting Type
    ///                                 |      &T
    ///   X       X                     |      &mut T
    ///   X                             |      &Cell<T>
    ///           X       X             |      Box<T>
    ///                           X     |      &[T]
    ///   X       X               X     |      &mut [T]
    ///           X       X       X     |      Box<[T]>
    fn determine_rust_type(permissions: &[PointerAccess]) -> RustPointerType {
        let [has_write, has_unique, has_free, has_offset_add, has_offset_sub]: [bool; 5] = ACCESSES
            .iter()
            .map(|access_type| permissions.contains(access_type))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        match (
            has_write,
            has_unique,
            has_free,
            has_offset_add,
            has_offset_sub,
        ) {
            // &T
            (false, false, false, false, false) => RustPointerType::ImmutableReference,
            // Write + Unique -> &mut T
            (true, true, false, false, false) => RustPointerType::MutableReference,
            // Write -> &Cell<T>
            (true, false, false, false, false) => RustPointerType::CellReference,
            // Unique + Free -> Box<T>
            (false, true, true, false, false) => RustPointerType::UniquePointer,
            // Offset -> &[T]
            (false, false, false, true, true)
            | (false, false, false, true, false)
            | (false, false, false, false, true) => RustPointerType::ImmutableSlice,
            // Write + Unique + Offset -> &mut [T]
            (true, true, false, true, true)
            | (true, true, false, true, false)
            | (true, true, false, false, true) => RustPointerType::MutableSlice,
            // Unique + Free + Offset -> Box<T>
            (false, true, true, true, true)
            | (false, true, true, true, false)
            | (false, true, true, false, true) => RustPointerType::UniqueSlicePointer,
            _ => RustPointerType::Undefined,
        }
    }
}

#[derive(Default)]
pub struct RawPointerSanitizer {
    pointers: HashMap<Ident, (TypePtr, Vec<PointerAccess>)>,
}

impl Visit<'_> for RawPointerSanitizer {
    fn visit_fn_arg(&mut self, arg: &FnArg) {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            if let syn::Pat::Ident(identifier) = &**pat {
                if let Type::Ptr(pointer) = &**ty {
                    // TODO: remove debug log
                    println!(
                        "Found raw pointer argument: {}: {}",
                        quote! { #identifier }.to_string(),
                        quote! { #pointer }.to_string()
                    );
                    self.pointers
                        .insert(identifier.ident.clone(), (pointer.clone(), Vec::new()));
                }
            }
        }
    }
}

impl Pass for RawPointerSanitizer {
    fn bind(&mut self, mut monad: MonadicAst) -> MonadicAst {
        self.visit_file(&mut monad.ast);
        // TODO: remove debug log
        self.pointers.iter().for_each(|(k, v)| println!("{:?}", k));
        monad
    }
}
