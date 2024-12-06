#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe fn insertion_sort(n: i32, p: *mut i32) {
    let mut i: i32 = 1 as i32;
    for i in 1..n {
        let tmp: i32 = *p.offset(i as isize);
        let mut j: i32 = i;
        while j > 0 as i32 && *p.offset((j - 1 as i32) as isize) > tmp {
            *p.offset(j as isize) = *p.offset((j - 1 as i32) as isize);
            j -= 1;
        }
        *p.offset(j as isize) = tmp;
    }
}
