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
    fn exit(_: i32) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Process {
    pub id: i32,
    pub burstTime: i32,
    pub priority: i32,
    pub arrivalTime: i32,
    pub isCompleted: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Queue {
    pub processes: [*mut Process; 10],
    pub front: i32,
    pub rear: i32,
}
#[no_mangle]
pub static mut queues: [Queue; 3] = [Queue {
    processes: [0 as *const Process as *mut Process; 10],
    front: 0,
    rear: 0,
}; 3];
#[no_mangle]
pub unsafe fn initializeQueue(mut q: *mut Queue) {
    (*q).front = -(1 as i32);
    (*q).rear = -(1 as i32);
}
#[no_mangle]
pub unsafe fn isEmpty(mut q: *mut Queue) -> i32 {
    return ((*q).front == -(1 as i32)) as i32;
}
#[no_mangle]
pub unsafe fn enqueue(mut q: *mut Queue, mut p: *mut Process) {
    if (*q).rear == 10 as i32 - 1 as i32 {
        printf(b"Error: Queue is full!\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if isEmpty(q) != 0 {
        (*q).front = 0 as i32;
    }
    (*q).rear += 1;
    (*q).processes[(*q).rear as usize] = p;
}
#[no_mangle]
pub unsafe fn dequeue(mut q: *mut Queue) -> *mut Process {
    if isEmpty(q) != 0 {
        return 0 as *mut Process;
    }
    let mut p: *mut Process = (*q).processes[(*q).front as usize];
    if (*q).front == (*q).rear {
        (*q).rear = -(1 as i32);
        (*q).front = (*q).rear;
    } else {
        (*q).front += 1;
        (*q).front;
    }
    return p;
}
#[no_mangle]
pub unsafe fn initializeScheduler(mut processes: *mut Process, mut numProcesses: i32) {
    let mut i: i32 = 0 as i32;
    while i < 3 as i32 {
        initializeQueue(&mut *queues.as_mut_ptr().offset(i as isize));
        i += 1;
    }
    let mut i_0: i32 = 0 as i32;
    for i_0 in 0..numProcesses {
        (*processes.offset(i_0 as isize)).priority = 0 as i32;
        (*processes.offset(i_0 as isize)).isCompleted = 0 as i32;
        enqueue(
            &mut *queues.as_mut_ptr().offset(0 as i32 as isize),
            &mut *processes.offset(i_0 as isize),
        );
    }
}
#[no_mangle]
pub unsafe fn runMLFQScheduler(mut processes: *mut Process, mut numProcesses: i32) {
    let mut currentTime: i32 = 0 as i32;
    loop {
        let mut allCompleted: i32 = 1 as i32;
        let mut i: i32 = 0 as i32;
        while i < 3 as i32 {
            while isEmpty(&mut *queues.as_mut_ptr().offset(i as isize)) == 0 {
                let mut currentProcess: *mut Process = dequeue(
                    &mut *queues.as_mut_ptr().offset(i as isize),
                );
                if (*currentProcess).isCompleted != 0 {
                    continue;
                }
                allCompleted = 0 as i32;
                let mut timeSlice: i32 = if (*currentProcess).burstTime < 4 as i32 {
                    (*currentProcess).burstTime
                } else {
                    4 as i32
                };
                printf(
                    b"Time %d: Running process %d (Queue %d) for %d units\n\0"
                        as *const u8 as *const i8,
                    currentTime,
                    (*currentProcess).id,
                    i,
                    timeSlice,
                );
                currentTime += timeSlice;
                (*currentProcess).burstTime -= timeSlice;
                if (*currentProcess).burstTime <= 0 as i32 {
                    (*currentProcess).isCompleted = 1 as i32;
                    printf(
                        b"Time %d: Process %d completed\n\0" as *const u8 as *const i8,
                        currentTime,
                        (*currentProcess).id,
                    );
                } else if i < 3 as i32 - 1 as i32 {
                    (*currentProcess).priority += 1;
                    (*currentProcess).priority;
                    enqueue(
                        &mut *queues.as_mut_ptr().offset((i + 1 as i32) as isize),
                        currentProcess,
                    );
                    printf(
                        b"Time %d: Process %d moved to Queue %d\n\0" as *const u8
                            as *const i8,
                        currentTime,
                        (*currentProcess).id,
                        i + 1 as i32,
                    );
                } else {
                    enqueue(
                        &mut *queues.as_mut_ptr().offset(i as isize),
                        currentProcess,
                    );
                }
            }
            i += 1;
        }
        if allCompleted != 0 {
            break;
        }
    };
}
unsafe fn main_0() -> i32 {
    let mut numProcesses: i32 = 0;
    printf(b"Enter the number of processes: \0" as *const u8 as *const i8);
    scanf(b"%d\0" as *const u8 as *const i8, &mut numProcesses as *mut i32);
    if numProcesses > 10 as i32 {
        printf(
            b"Error: Number of processes exceeds maximum limit (%d).\n\0" as *const u8
                as *const i8,
            10 as i32,
        );
        return 1 as i32;
    }
    let mut processes: [Process; 10] = [Process {
        id: 0,
        burstTime: 0,
        priority: 0,
        arrivalTime: 0,
        isCompleted: 0,
    }; 10];
    let mut i: i32 = 0 as i32;
    for i in 0..numProcesses {
        printf(
            b"Enter burst time and arrival time for process %d: \0" as *const u8
                as *const i8,
            i,
        );
        scanf(
            b"%d %d\0" as *const u8 as *const i8,
            &mut (*processes.as_mut_ptr().offset(i as isize)).burstTime as *mut i32,
            &mut (*processes.as_mut_ptr().offset(i as isize)).arrivalTime as *mut i32,
        );
        processes[i as usize].id = i;
    }
    initializeScheduler(processes.as_mut_ptr(), numProcesses);
    runMLFQScheduler(processes.as_mut_ptr(), numProcesses);
    return 0 as i32;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
