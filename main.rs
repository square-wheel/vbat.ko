#![no_std]
#![allow(improper_ctypes)]
#![feature(lang_items)]

#[lang="sized"]
trait Sized {}

extern {
    pub fn idiv(a: i64, b: i64) -> i64;
}

#[no_mangle]
pub unsafe fn rust_percent(now: i64, full: i64) -> i64 {
    return idiv(now * 100, full);
}
