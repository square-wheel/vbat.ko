#![feature(lang_items)]
#![feature(intrinsics)]

#![no_std]
#![allow(improper_ctypes)]

use power_supply::power_supply_property;
use power_supply::power_supply_type;
mod power_supply;

mod zero;

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
    now * 100 / full
}
