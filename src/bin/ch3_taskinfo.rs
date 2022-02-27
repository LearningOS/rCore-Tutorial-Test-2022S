#![no_std]
#![no_main]

extern crate user_lib;
use user_lib::{
    get_time, println, task_info, TaskInfo, SYSCALL_EXIT, SYSCALL_GETTIMEOFDAY, SYSCALL_TASK_INFO,
    SYSCALL_WRITE, SYSCALL_YIELD,
};
use user_lib::{write, STDOUT};
const DATA_STRING: &str = "string from data section\n";

#[no_mangle]
pub fn main() -> usize {
    let info = TaskInfo::new();

    get_time();
    get_time();
    get_time();
    assert_eq!(0, task_info(&info));
    // Only one test case in TEST2
    assert_eq!(0, info.id);
    info.syscall_ids
        .iter()
        .enumerate()
        .for_each(|(i, &id)| match id {
            SYSCALL_GETTIMEOFDAY => assert_eq!(3, info.syscall_times[i]),
            SYSCALL_TASK_INFO => assert_eq!(1, info.syscall_times[i]),
            SYSCALL_WRITE => assert_eq!(0, info.syscall_times[i]),
            SYSCALL_YIELD => assert_eq!(0, info.syscall_times[i]),
            SYSCALL_EXIT => assert_eq!(0, info.syscall_times[i]),
            _ => assert_eq!(0, 1),
        });

    // Why write 2 times...
    println!("string from task info test\n");
    assert_eq!(0, task_info(&info));
    info.syscall_ids
        .iter()
        .enumerate()
        .for_each(|(i, &id)| match id {
            SYSCALL_GETTIMEOFDAY => assert_eq!(3, info.syscall_times[i]),
            SYSCALL_TASK_INFO => assert_eq!(2, info.syscall_times[i]),
            SYSCALL_WRITE => assert_eq!(2, info.syscall_times[i]),
            SYSCALL_YIELD => assert_eq!(0, info.syscall_times[i]),
            SYSCALL_EXIT => assert_eq!(0, info.syscall_times[i]),
            _ => assert_eq!(0, 1),
        });
    0
}
