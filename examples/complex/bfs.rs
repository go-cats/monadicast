#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bfs(
    mut graph: *mut [libc::c_int; 100],
    mut numNodes: libc::c_int,
    mut startNode: libc::c_int,
) {
    let mut visited: [libc::c_int; 100] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut queue: [libc::c_int; 100] = [0; 100];
    let mut front: libc::c_int = 0 as libc::c_int;
    let mut rear: libc::c_int = 0 as libc::c_int;
    let fresh0 = rear;
    rear = rear + 1;
    queue[fresh0 as usize] = startNode;
    visited[startNode as usize] = 1 as libc::c_int;
    printf(b"BFS Traversal: \0" as *const u8 as *const libc::c_char);
    while front < rear {
        let fresh1 = front;
        front = front + 1;
        let mut currentNode: libc::c_int = queue[fresh1 as usize];
        printf(b"%d \0" as *const u8 as *const libc::c_char, currentNode);
        let mut neighbor: libc::c_int = 0 as libc::c_int;
        while neighbor < numNodes {
            if (*graph.offset(currentNode as isize))[neighbor as usize] != 0
                && visited[neighbor as usize] == 0
            {
                let fresh2 = rear;
                rear = rear + 1;
                queue[fresh2 as usize] = neighbor;
                visited[neighbor as usize] = 1 as libc::c_int;
            }
            neighbor += 1;
            neighbor;
        }
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    let mut numNodes: libc::c_int = 0;
    let mut startNode: libc::c_int = 0;
    let mut graph: [[libc::c_int; 100]; 100] = [[0; 100]; 100];
    printf(b"Enter the number of nodes: \0" as *const u8 as *const libc::c_char);
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numNodes as *mut libc::c_int,
    );
    printf(b"Enter the adjacency matrix:\n\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numNodes {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < numNodes {
            scanf(
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut *(*graph.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(j as isize) as *mut libc::c_int,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    printf(b"Enter the starting node: \0" as *const u8 as *const libc::c_char);
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut startNode as *mut libc::c_int,
    );
    bfs(graph.as_mut_ptr(), numNodes, startNode);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
