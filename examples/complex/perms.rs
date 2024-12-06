#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct User {
    pub id: libc::c_int,
    pub permissions: libc::c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn checkPermissions(
    mut users: *mut User,
    mut numUsers: libc::c_int,
) {
    printf(
        b"Checking permissions for all users:\n\0" as *const u8 as *const libc::c_char,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numUsers {
        printf(
            b"User ID: %d\n\0" as *const u8 as *const libc::c_char,
            (*users.offset(i as isize)).id,
        );
        printf(b"  Permissions: \0" as *const u8 as *const libc::c_char);
        if (*users.offset(i as isize)).permissions
            & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
        {
            printf(b"Read \0" as *const u8 as *const libc::c_char);
        }
        if (*users.offset(i as isize)).permissions
            & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0
        {
            printf(b"Write \0" as *const u8 as *const libc::c_char);
        }
        if (*users.offset(i as isize)).permissions
            & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
        {
            printf(b"Execute \0" as *const u8 as *const libc::c_char);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn modifyPermissions(mut user: *mut User) {
    printf(
        b"\nModify permissions for User ID %d:\n\0" as *const u8 as *const libc::c_char,
        (*user).id,
    );
    printf(
        b"Enter a new permission number (binary format, e.g., 111 for Read/Write/Execute): \0"
            as *const u8 as *const libc::c_char,
    );
    let mut newPermissions: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        scanf(
            b"%u\0" as *const u8 as *const libc::c_char,
            &mut newPermissions as *mut libc::c_uint,
        );
        if newPermissions <= 7 as libc::c_int as libc::c_uint {
            break;
        }
        printf(
            b"Invalid input! Enter a number between 0 and 7: \0" as *const u8
                as *const libc::c_char,
        );
    }
    (*user).permissions = newPermissions;
    printf(b"Permissions updated.\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    let mut numUsers: libc::c_int = 0;
    printf(b"Enter the number of users: \0" as *const u8 as *const libc::c_char);
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numUsers as *mut libc::c_int,
    );
    let vla = numUsers as usize;
    let mut users: Vec::<User> = ::std::vec::from_elem(
        User { id: 0, permissions: 0 },
        vla,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numUsers {
        (*users.as_mut_ptr().offset(i as isize)).id = i + 1 as libc::c_int;
        printf(
            b"Enter permissions for User %d (as a number, 0-7): \0" as *const u8
                as *const libc::c_char,
            (*users.as_mut_ptr().offset(i as isize)).id,
        );
        loop {
            scanf(
                b"%u\0" as *const u8 as *const libc::c_char,
                &mut (*users.as_mut_ptr().offset(i as isize)).permissions
                    as *mut libc::c_uint,
            );
            if (*users.as_mut_ptr().offset(i as isize)).permissions
                <= 7 as libc::c_int as libc::c_uint
            {
                break;
            }
            printf(
                b"Invalid input! Enter a number between 0 and 7: \0" as *const u8
                    as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
    checkPermissions(users.as_mut_ptr(), numUsers);
    let mut targetUserID: libc::c_int = 0;
    printf(
        b"\nEnter the User ID to modify permissions: \0" as *const u8
            as *const libc::c_char,
    );
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut targetUserID as *mut libc::c_int,
    );
    while targetUserID < 1 as libc::c_int || targetUserID > numUsers {
        printf(
            b"Invalid User ID! Please enter a valid ID (1 to %d): \0" as *const u8
                as *const libc::c_char,
            numUsers,
        );
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut targetUserID as *mut libc::c_int,
        );
    }
    modifyPermissions(
        &mut *users.as_mut_ptr().offset((targetUserID - 1 as libc::c_int) as isize),
    );
    printf(b"\nPermissions after modification:\n\0" as *const u8 as *const libc::c_char);
    checkPermissions(users.as_mut_ptr(), numUsers);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
