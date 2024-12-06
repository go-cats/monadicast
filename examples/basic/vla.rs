#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn vla_index(
    mut a: libc::c_int,
    mut xs: *mut libc::c_int,
    mut i: libc::c_int,
) -> libc::c_int {
    let vla = a as usize;
    return xs.offset(i as isize * vla as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vla_alloc(mut a: libc::c_int, mut b: libc::c_int) {
    let vla = a as usize;
    let vla_0 = b as usize;
    let mut xs: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla * vla_0);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < a {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < b {
            *xs
                .as_mut_ptr()
                .offset(i as isize * vla_0 as isize)
                .offset(j as isize) = 1 as libc::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}