#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::{mutex_blocking_create, mutex_lock, mutex_unlock};

#[no_mangle]
pub fn main() -> i32 {
    let mid = mutex_blocking_create() as usize;
    assert_eq!(mutex_lock(mid), 0);
    assert_eq!(mutex_lock(mid), -1);
    mutex_unlock(mid);
    println!("deadlock test1 OK!");
    0
}
