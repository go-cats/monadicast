//! See https://immunant.com/blog/2023/03/lifting/ for more information on
//! pointer derivation graph (PDG) matching.

use crate::monad::ast::Pass;
use crate::MonadicAst;
use quote::quote;
use std::collections::HashMap;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{ExprMethodCall, FnArg, Ident, Local, Pat, PatIdent, PatType, Type, TypePtr};

/// Represents a permission that a raw pointer *p will need at the point in the
/// program p is defined and used.
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
    /// Keeps track of pointer variables and their access permissions.
    pointers: HashMap<Ident, (TypePtr, Vec<PointerAccess>)>,
    /// Mapping between the pointer variables and their memory safe equivalent types.
    types: HashMap<Ident, RustPointerType>,
}

impl RawPointerSanitizer {
    fn record_if_pointer(&mut self, pat: &Pat, ty: &Type) {
        match (pat, ty) {
            (
                Pat::Ident(PatIdent {
                    mutability: _,
                    ident,
                    ..
                }),
                Type::Ptr(pointer),
            ) => {
                self.pointers
                    .insert(ident.clone(), (pointer.clone(), Vec::new()));
            }
            _ => {}
        }
    }
}

impl Visit<'_> for RawPointerSanitizer {
    /// Inspects a function argument and adds it to the `pointers` map if it is a
    /// raw pointer type.
    fn visit_fn_arg(&mut self, arg: &FnArg) {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            self.record_if_pointer(&**pat, &**ty)
        }
        syn::visit::visit_fn_arg(self, arg)
    }

    /// Inspects a local variable declaration and adds it to the `pointers` map if it
    /// is a raw pointer type declaration.
    fn visit_local(&mut self, assignment: &Local) {
        if let Pat::Type(PatType { pat, ty, .. }) = &assignment.pat {
            self.record_if_pointer(&**pat, &**ty)
        }
        syn::visit::visit_local(self, assignment)
    }

    /// Inspects a method call, updating the pointer accesses mapping if the call is a
    /// raw pointer access.
    fn visit_expr_method_call(&mut self, expr: &ExprMethodCall) {
        let ExprMethodCall {
            method, receiver, ..
        } = expr;
        println!("Visit - {}", quote! { #expr }.to_string());
        println!(" -- {}", quote! { #method }.to_string());
        println!(" -! {}", quote! { #receiver }.to_string());
        syn::visit::visit_expr_method_call(self, expr)
    }
}

impl VisitMut for RawPointerSanitizer {
    // TODO
}

impl Pass for RawPointerSanitizer {
    fn bind(&mut self, mut monad: MonadicAst) -> MonadicAst {
        // Identifies the function arguments and local variables that are raw pointers,
        // TODO - then computing the access permissions needed from how they're used.
        self.visit_file(&mut monad.ast);

        // TODO - Replaces the types of the raw pointer variables with their memory safe Rust
        //      - equivalents, computed from their access permissions. Updates the accesses of
        //      - the updated variables, as necessary.
        self.visit_file_mut(&mut monad.ast);

        monad
    }
}
