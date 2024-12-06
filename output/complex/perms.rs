#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn scanf(_: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct User {
    pub id: i32,
    pub permissions: u32,
}
#[no_mangle]
pub unsafe fn checkPermissions(mut users: *mut User, mut numUsers: i32) {
    printf(b"Checking permissions for all users:\n\0" as *const u8 as *const i8);
    let mut i: i32 = 0 as i32;
    for i in 0..numUsers {
        printf(
            b"User ID: %d\n\0" as *const u8 as *const i8,
            (*users.offset(i as isize)).id,
        );
        printf(b"  Permissions: \0" as *const u8 as *const i8);
        if (*users.offset(i as isize)).permissions & ((1 as i32) << 2 as i32) as u32 != 0
        {
            printf(b"Read \0" as *const u8 as *const i8);
        }
        if (*users.offset(i as isize)).permissions & ((1 as i32) << 1 as i32) as u32 != 0
        {
            printf(b"Write \0" as *const u8 as *const i8);
        }
        if (*users.offset(i as isize)).permissions & ((1 as i32) << 0 as i32) as u32 != 0
        {
            printf(b"Execute \0" as *const u8 as *const i8);
        }
        printf(b"\n\0" as *const u8 as *const i8);
    }
}
#[no_mangle]
pub unsafe fn modifyPermissions(mut user: *mut User) {
    printf(
        b"\nModify permissions for User ID %d:\n\0" as *const u8 as *const i8,
        (*user).id,
    );
    printf(
        b"Enter a new permission number (binary format, e.g., 111 for Read/Write/Execute): \0"
            as *const u8 as *const i8,
    );
    let mut newPermissions: u32 = 0 as i32 as u32;
    loop {
        scanf(b"%u\0" as *const u8 as *const i8, &mut newPermissions as *mut u32);
        if newPermissions <= 7 as i32 as u32 {
            break;
        }
        printf(
            b"Invalid input! Enter a number between 0 and 7: \0" as *const u8
                as *const i8,
        );
    }
    (*user).permissions = newPermissions;
    printf(b"Permissions updated.\n\0" as *const u8 as *const i8);
}
unsafe fn main_0() -> i32 {
    let mut numUsers: i32 = 0;
    printf(b"Enter the number of users: \0" as *const u8 as *const i8);
    scanf(b"%d\0" as *const u8 as *const i8, &mut numUsers as *mut i32);
    let vla = numUsers as usize;
    let mut users: Vec<User> = ::std::vec::from_elem(
        User { id: 0, permissions: 0 },
        vla,
    );
    let mut i: i32 = 0 as i32;
    for i in 0..numUsers {
        (*users.as_mut_ptr().offset(i as isize)).id = i + 1 as i32;
        printf(
            b"Enter permissions for User %d (as a number, 0-7): \0" as *const u8
                as *const i8,
            (*users.as_mut_ptr().offset(i as isize)).id,
        );
        loop {
            scanf(
                b"%u\0" as *const u8 as *const i8,
                &mut (*users.as_mut_ptr().offset(i as isize)).permissions as *mut u32,
            );
            if (*users.as_mut_ptr().offset(i as isize)).permissions <= 7 as i32 as u32 {
                break;
            }
            printf(
                b"Invalid input! Enter a number between 0 and 7: \0" as *const u8
                    as *const i8,
            );
        }
    }
    checkPermissions(users.as_mut_ptr(), numUsers);
    let mut targetUserID: i32 = 0;
    printf(b"\nEnter the User ID to modify permissions: \0" as *const u8 as *const i8);
    scanf(b"%d\0" as *const u8 as *const i8, &mut targetUserID as *mut i32);
    while targetUserID < 1 as i32 || targetUserID > numUsers {
        printf(
            b"Invalid User ID! Please enter a valid ID (1 to %d): \0" as *const u8
                as *const i8,
            numUsers,
        );
        scanf(b"%d\0" as *const u8 as *const i8, &mut targetUserID as *mut i32);
    }
    modifyPermissions(
        &mut *users.as_mut_ptr().offset((targetUserID - 1 as i32) as isize),
    );
    printf(b"\nPermissions after modification:\n\0" as *const u8 as *const i8);
    checkPermissions(users.as_mut_ptr(), numUsers);
    return 0 as i32;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
