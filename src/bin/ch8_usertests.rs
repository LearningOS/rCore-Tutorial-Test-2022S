#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "ch2b_hello_world\0",
    "ch2b_power_3\0",
    "ch2b_power_5\0",
    "ch2b_power_7\0",
    "ch3b_yield0\0",
    "ch3b_yield1\0",
    "ch3b_yield2\0",
    "ch3b_sleep\0",
    "ch3b_sleep1\0",
    "ch4_mmap0\0",
    "ch4_mmap1\0",
    "ch4_mmap2\0",
    "ch4_mmap3\0",
    "ch4_unmap\0",
    "ch4_unmap2\0",
    "ch5b_forktest2\0",
    "ch5_spawn0\0",
    "ch5_spawn1\0",
    "ch5_setprio\0",
    "ch5_stride\0",
    "ch6_file0\0",
    "ch6_file1\0",
    "ch6_file2\0",
    "ch6_file3\0",
];

use user_lib::{exec, fork, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    for test in TESTS {
        println!("Usertests: Running {}", test);
        let pid = fork();
        if pid == 0 {
            exec(*test, &[core::ptr::null::<u8>()]);
            panic!("unreachable!");
        } else {
            let mut exit_code: i32 = Default::default();
            let wait_pid = waitpid(pid as usize, &mut exit_code);
            assert_eq!(pid, wait_pid);
            println!(
                "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
                test, pid, exit_code
            );
        }
    }
    println!("Usertests passed!");
    0
}
