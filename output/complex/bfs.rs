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
    fn printf(_: *const i8, _: ...) -> i32;
    fn scanf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub unsafe fn bfs(mut graph: *mut [i32; 100], mut numNodes: i32, mut startNode: i32) {
    let mut visited: [i32; 100] = [
        0 as i32,
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
    let mut queue: [i32; 100] = [0; 100];
    let mut front: i32 = 0 as i32;
    let mut rear: i32 = 0 as i32;
    let fresh0 = rear;
    rear = rear + 1;
    queue[fresh0 as usize] = startNode;
    visited[startNode as usize] = 1 as i32;
    printf(b"BFS Traversal: \0" as *const u8 as *const i8);
    for front in 0..0 {
        let fresh1 = front;
        let mut currentNode: i32 = queue[fresh1 as usize];
        printf(b"%d \0" as *const u8 as *const i8, currentNode);
        let mut neighbor: i32 = 0 as i32;
        for neighbor in 0..numNodes {
            if (*graph.offset(currentNode as isize))[neighbor as usize] != 0
                && visited[neighbor as usize] == 0
            {
                let fresh2 = rear;
                rear = rear + 1;
                queue[fresh2 as usize] = neighbor;
                visited[neighbor as usize] = 1 as i32;
            }
        }
    }
    printf(b"\n\0" as *const u8 as *const i8);
}
unsafe fn main_0() -> i32 {
    let mut numNodes: i32 = 0;
    let mut startNode: i32 = 0;
    let mut graph: [[i32; 100]; 100] = [[0; 100]; 100];
    printf(b"Enter the number of nodes: \0" as *const u8 as *const i8);
    scanf(b"%d\0" as *const u8 as *const i8, &mut numNodes as *mut i32);
    printf(b"Enter the adjacency matrix:\n\0" as *const u8 as *const i8);
    let mut i: i32 = 0 as i32;
    for i in 0..numNodes {
        let mut j: i32 = 0 as i32;
        for j in 0..numNodes {
            scanf(
                b"%d\0" as *const u8 as *const i8,
                &mut *(*graph.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(j as isize) as *mut i32,
            );
        }
    }
    printf(b"Enter the starting node: \0" as *const u8 as *const i8);
    scanf(b"%d\0" as *const u8 as *const i8, &mut startNode as *mut i32);
    bfs(graph.as_mut_ptr(), numNodes, startNode);
    return 0 as i32;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
