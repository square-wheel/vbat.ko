#![no_std]
#![allow(improper_ctypes)]
#![feature(lang_items)]

use power_supply::power_supply_property;
use power_supply::power_supply_type;
mod power_supply;

#[lang="sized"]
trait Sized {}

#[lang="sync"]
trait Sync {}

extern {
    pub fn idiv(a: i64, b: i64) -> i64;
}

#[no_mangle]
pub static RUST_VBAT_NUM_PROPS: int = 3;

#[no_mangle]
pub static RUST_VBAT_PROPS: [power_supply_property, ..3] = [

    power_supply_property::CAPACITY,
    power_supply_property::ENERGY_FULL,
    power_supply_property::ENERGY_NOW,
];

#[no_mangle]
pub static RUST_VBAT_NAME: &'static str = "VBAT\0";

#[no_mangle]
pub static RUST_VBAT_TYPE: power_supply_type = power_supply_type::BATTERY;

#[no_mangle]
pub unsafe fn rust_percent(now: i64, full: i64) -> i64 {
    idiv(now * 100, full)
}
