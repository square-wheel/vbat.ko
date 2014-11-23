#![no_std]

extern crate core;

#[no_mangle]
pub fn rust_percent(now: int, full: int) -> int {
    return now * 100 / full;
    // if you want to make it compile:
    // return 42;
}
