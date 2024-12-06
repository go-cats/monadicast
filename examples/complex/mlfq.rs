#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Process {
    pub id: libc::c_int,
    pub burstTime: libc::c_int,
    pub priority: libc::c_int,
    pub arrivalTime: libc::c_int,
    pub isCompleted: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Queue {
    pub processes: [*mut Process; 10],
    pub front: libc::c_int,
    pub rear: libc::c_int,
}
#[no_mangle]
pub static mut queues: [Queue; 3] = [Queue {
    processes: [0 as *const Process as *mut Process; 10],
    front: 0,
    rear: 0,
}; 3];
#[no_mangle]
pub unsafe extern "C" fn initializeQueue(mut q: *mut Queue) {
    (*q).front = -(1 as libc::c_int);
    (*q).rear = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn isEmpty(mut q: *mut Queue) -> libc::c_int {
    return ((*q).front == -(1 as libc::c_int)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn enqueue(mut q: *mut Queue, mut p: *mut Process) {
    if (*q).rear == 10 as libc::c_int - 1 as libc::c_int {
        printf(b"Error: Queue is full!\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if isEmpty(q) != 0 {
        (*q).front = 0 as libc::c_int;
    }
    (*q).rear += 1;
    (*q).processes[(*q).rear as usize] = p;
}
#[no_mangle]
pub unsafe extern "C" fn dequeue(mut q: *mut Queue) -> *mut Process {
    if isEmpty(q) != 0 {
        return 0 as *mut Process;
    }
    let mut p: *mut Process = (*q).processes[(*q).front as usize];
    if (*q).front == (*q).rear {
        (*q).rear = -(1 as libc::c_int);
        (*q).front = (*q).rear;
    } else {
        (*q).front += 1;
        (*q).front;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn initializeScheduler(
    mut processes: *mut Process,
    mut numProcesses: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        initializeQueue(&mut *queues.as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < numProcesses {
        (*processes.offset(i_0 as isize)).priority = 0 as libc::c_int;
        (*processes.offset(i_0 as isize)).isCompleted = 0 as libc::c_int;
        enqueue(
            &mut *queues.as_mut_ptr().offset(0 as libc::c_int as isize),
            &mut *processes.offset(i_0 as isize),
        );
        i_0 += 1;
        i_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn runMLFQScheduler(
    mut processes: *mut Process,
    mut numProcesses: libc::c_int,
) {
    let mut currentTime: libc::c_int = 0 as libc::c_int;
    loop {
        let mut allCompleted: libc::c_int = 1 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            while isEmpty(&mut *queues.as_mut_ptr().offset(i as isize)) == 0 {
                let mut currentProcess: *mut Process = dequeue(
                    &mut *queues.as_mut_ptr().offset(i as isize),
                );
                if (*currentProcess).isCompleted != 0 {
                    continue;
                }
                allCompleted = 0 as libc::c_int;
                let mut timeSlice: libc::c_int = if (*currentProcess).burstTime
                    < 4 as libc::c_int
                {
                    (*currentProcess).burstTime
                } else {
                    4 as libc::c_int
                };
                printf(
                    b"Time %d: Running process %d (Queue %d) for %d units\n\0"
                        as *const u8 as *const libc::c_char,
                    currentTime,
                    (*currentProcess).id,
                    i,
                    timeSlice,
                );
                currentTime += timeSlice;
                (*currentProcess).burstTime -= timeSlice;
                if (*currentProcess).burstTime <= 0 as libc::c_int {
                    (*currentProcess).isCompleted = 1 as libc::c_int;
                    printf(
                        b"Time %d: Process %d completed\n\0" as *const u8
                            as *const libc::c_char,
                        currentTime,
                        (*currentProcess).id,
                    );
                } else if i < 3 as libc::c_int - 1 as libc::c_int {
                    (*currentProcess).priority += 1;
                    (*currentProcess).priority;
                    enqueue(
                        &mut *queues
                            .as_mut_ptr()
                            .offset((i + 1 as libc::c_int) as isize),
                        currentProcess,
                    );
                    printf(
                        b"Time %d: Process %d moved to Queue %d\n\0" as *const u8
                            as *const libc::c_char,
                        currentTime,
                        (*currentProcess).id,
                        i + 1 as libc::c_int,
                    );
                } else {
                    enqueue(
                        &mut *queues.as_mut_ptr().offset(i as isize),
                        currentProcess,
                    );
                }
            }
            i += 1;
            i;
        }
        if allCompleted != 0 {
            break;
        }
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut numProcesses: libc::c_int = 0;
    printf(b"Enter the number of processes: \0" as *const u8 as *const libc::c_char);
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numProcesses as *mut libc::c_int,
    );
    if numProcesses > 10 as libc::c_int {
        printf(
            b"Error: Number of processes exceeds maximum limit (%d).\n\0" as *const u8
                as *const libc::c_char,
            10 as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    let mut processes: [Process; 10] = [Process {
        id: 0,
        burstTime: 0,
        priority: 0,
        arrivalTime: 0,
        isCompleted: 0,
    }; 10];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numProcesses {
        printf(
            b"Enter burst time and arrival time for process %d: \0" as *const u8
                as *const libc::c_char,
            i,
        );
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut (*processes.as_mut_ptr().offset(i as isize)).burstTime
                as *mut libc::c_int,
            &mut (*processes.as_mut_ptr().offset(i as isize)).arrivalTime
                as *mut libc::c_int,
        );
        processes[i as usize].id = i;
        i += 1;
        i;
    }
    initializeScheduler(processes.as_mut_ptr(), numProcesses);
    runMLFQScheduler(processes.as_mut_ptr(), numProcesses);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
