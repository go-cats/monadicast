//! See https://immunant.com/blog/2023/03/lifting/ for more information on
//! pointer derivation graph (PDG) matching.

use crate::monad::ast::Pass;
use crate::MonadicAst;
use std::collections::{HashMap, HashSet};
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{
    Expr, ExprAssign, ExprMethodCall, ExprPath, ExprUnary, File, FnArg, Ident, Local, Pat,
    PatIdent, PatType, Type, TypePtr, UnOp,
};

/// Represents a permission that a raw pointer *p will need at the point in the
/// program p is defined and used.
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
enum PointerAccess {
    Write,  // The program writes to the pointee.
    Unique, // The pointer is the only way to access the given memory location.
    Free,   // The pointer will eventually be passed to free.
    Offset, // We'll add/subtract an offset to the pointer, e.g. array element access.
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
    PointerAccess::Offset,
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
        let [has_write, has_unique, has_free, has_offset]: [bool; 4] = ACCESSES
            .iter()
            .map(|access_type| permissions.contains(access_type))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        match (has_write, has_unique, has_free, has_offset) {
            // &T
            (false, false, false, false) => RustPointerType::ImmutableReference,
            // Write + Unique -> &mut T
            (true, true, false, false) => RustPointerType::MutableReference,
            // Write -> &Cell<T>
            (true, false, false, false) => RustPointerType::CellReference,
            // Unique + Free -> Box<T>
            (false, true, true, false) => RustPointerType::UniquePointer,
            // Offset -> &[T]
            (false, false, false, true) => RustPointerType::ImmutableSlice,
            // Write + Unique + Offset -> &mut [T]
            (true, true, false, true) => RustPointerType::MutableSlice,
            // Unique + Free + Offset -> Box<T>
            (false, true, true, true) => RustPointerType::UniqueSlicePointer,
            _ => RustPointerType::Undefined,
        }
    }
}

#[derive(Default)]
enum TypeMappingStateMachine {
    /// Still identifying usages of raw pointers, or the process of mapping them
    /// to their appropriate Rust safe reference type hasn't started yet.
    #[default]
    Uninitialized,
    /// Currently in the process of mapping identifiers to their appropriate Rust
    /// safe reference types.
    Computing(HashMap<Ident, RustPointerType>),
    /// All raw pointer identifiers have been mapped to their appropriate Rust
    /// safe reference type.
    Initialized(HashMap<Ident, RustPointerType>),
}

#[derive(Default)]
pub struct RawPointerSanitizer {
    /// Keeps track of pointer variables and their access permissions.
    pointers: HashMap<Ident, (TypePtr, HashSet<PointerAccess>)>,
    /// Mapping between the pointer variables and their memory safe equivalent types.
    types: TypeMappingStateMachine,
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
                    .insert(ident.clone(), (pointer.clone(), HashSet::new()));
            }
            _ => {}
        }
    }

    fn identify_raw_pointer_args(&mut self, ast: &mut File) {
        self.visit_file(ast);

        // TODO(eyoon): delete debug log
        println!(
            "{:?}",
            &self
                .pointers
                .iter()
                .map(|(id, (_, set))| (id.to_string(), set))
                .collect::<Vec<_>>()
        );

        // Advance state from 'Uninitialized' to 'Computing'
        match self.types {
            TypeMappingStateMachine::Uninitialized => {
                self.types = TypeMappingStateMachine::Computing(HashMap::new())
            }
            _ => panic!("Must be in Uninitialized state"),
        }
    }

    fn compute_equivalent_safe_types(&mut self) {
        // TODO: compute type equivalents

        // Advance state from `Computing` to `Initialized`.
        let old_state = std::mem::replace(&mut self.types, TypeMappingStateMachine::Uninitialized);
        match old_state {
            TypeMappingStateMachine::Computing(map) => {
                self.types = TypeMappingStateMachine::Initialized(map)
            }
            _ => {
                let _ = std::mem::replace(&mut self.types, old_state);
                panic!("Must be in Computing state")
            }
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

    /// Inspects assignment instructions for lvalue pointer writes, updating the access
    /// map if a raw pointer write access is identified.
    ///
    /// Assumptions: (which are probably incorrect)
    /// - Raw pointers are never directly used as rvalues, and any rvalue expression will
    ///   contain a pointer dereference if it contains a raw pointer value.
    /// - Raw pointer variables are not reassigned to a different pointer when in lvalue
    ///   expressions.
    fn visit_expr_assign(&mut self, assign: &'_ ExprAssign) {
        fn access_set_if_pointer_access<'expr, 'vis>(
            input_expr: &'expr Box<Expr>,
            pointers: &'vis mut HashMap<Ident, (TypePtr, HashSet<PointerAccess>)>,
        ) -> Option<&'vis mut HashSet<PointerAccess>> {
            match input_expr.as_ref() {
                Expr::MethodCall(ExprMethodCall {
                                     method: _, receiver, ..
                                 }) =>
                    {
                        access_set_if_raw_ptr(receiver, pointers)
                    }
                _ => None
            }
        }

        // Identify lvalue raw pointer accesses.
        expr_if_unary_deref(&assign.left).map(|expr| {
            access_set_if_pointer_access(expr, &mut self.pointers)
                .map(|access_set| {
                    // *p = ...
                    access_set.insert(PointerAccess::Write);
                })
        });

        syn::visit::visit_expr_assign(self, assign)
    }

    /// Inspects method calls, updating the pointer access map if a raw pointer
    /// offset access is identified.
    fn visit_expr_method_call(&mut self, i: &'_ ExprMethodCall) {
        let ExprMethodCall { method, receiver, .. } = i;
        access_set_if_raw_ptr(receiver, &mut self.pointers)
            .map(|access_set| {
                if is_offset(method) {
                    access_set.insert(PointerAccess::Offset);
                }
            });

        syn::visit::visit_expr_method_call(self, i)
    }
}


impl VisitMut for RawPointerSanitizer {
    // TODO
}


/// If the given receiver `p` exists in the pointer map, return a mutable reference
/// to its access set pointers[p].1
fn access_set_if_raw_ptr<'expr, 'vis>(receiver: &'expr Box<Expr>, pointers: &'vis mut HashMap<Ident, (TypePtr, HashSet<PointerAccess>)>,
) -> Option<&'vis mut HashSet<PointerAccess>> {
    match receiver.as_ref() {
        Expr::Path(ExprPath { qself, path, .. }) => {
            if qself.is_some() {
                return None;
            }
            let ident = &path.segments.last().unwrap().ident;
            pointers.get_mut(ident).map(|(_, access_set)| {
                access_set
            })
        }
        _ => None
    }
}

/// If input_expr is *(inner), return Some(inner) and None otherwise.
fn expr_if_unary_deref(input_expr: &Box<Expr>) -> Option<&Box<Expr>> {
    if let Expr::Unary(ExprUnary { op, expr, .. }) = input_expr.as_ref() {
        if let UnOp::Deref(_) = op {
            return Some(expr);
        }
    }
    None
}

fn is_offset(ident: &Ident) -> bool {
    ident.to_string().eq("offset")
}

impl Pass for RawPointerSanitizer {
    fn bind(&mut self, mut monad: MonadicAst) -> MonadicAst {
        self.identify_raw_pointer_args(&mut monad.ast);
        self.compute_equivalent_safe_types();

        // TODO - Replaces the types of the raw pointer variables with their memory safe Rust
        //      - equivalents, computed from their access permissions. Updates the accesses of
        //      - the updated variables, as necessary.
        self.visit_file_mut(&mut monad.ast);

        monad
    }
}
