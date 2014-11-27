#![no_std]
#![allow(improper_ctypes)]
#![feature(lang_items)]

#[lang="sized"]
trait Sized {}

#[lang="sync"]
trait Sync {}

#[repr(C)]
pub enum power_supply_property {
    /* Properties of type `int' */
    STATUS = 0,
    CHARGE_TYPE,
    HEALTH,
    PRESENT,
    ONLINE,
    AUTHENTIC,
    TECHNOLOGY,
    CYCLE_COUNT,
    VOLTAGE_MAX,
    VOLTAGE_MIN,
    VOLTAGE_MAX_DESIGN,
    VOLTAGE_MIN_DESIGN,
    VOLTAGE_NOW,
    VOLTAGE_AVG,
    VOLTAGE_OCV,
    CURRENT_MAX,
    CURRENT_NOW,
    CURRENT_AVG,
    POWER_NOW,
    POWER_AVG,
    CHARGE_FULL_DESIGN,
    CHARGE_EMPTY_DESIGN,
    CHARGE_FULL,
    CHARGE_EMPTY,
    CHARGE_NOW,
    CHARGE_AVG,
    CHARGE_COUNTER,
    CONSTANT_CHARGE_CURRENT,
    CONSTANT_CHARGE_CURRENT_MAX,
    CONSTANT_CHARGE_VOLTAGE,
    CONSTANT_CHARGE_VOLTAGE_MAX,
    CHARGE_CONTROL_LIMIT,
    CHARGE_CONTROL_LIMIT_MAX,
    INPUT_CURRENT_LIMIT,
    ENERGY_FULL_DESIGN,
    ENERGY_EMPTY_DESIGN,
    ENERGY_FULL,
    ENERGY_EMPTY,
    ENERGY_NOW,
    ENERGY_AVG,
    CAPACITY, /* in percents! */
    CAPACITY_ALERT_MIN, /* in percents! */
    CAPACITY_ALERT_MAX, /* in percents! */
    CAPACITY_LEVEL,
    TEMP,
    TEMP_MAX,
    TEMP_MIN,
    TEMP_ALERT_MIN,
    TEMP_ALERT_MAX,
    TEMP_AMBIENT,
    TEMP_AMBIENT_ALERT_MIN,
    TEMP_AMBIENT_ALERT_MAX,
    TIME_TO_EMPTY_NOW,
    TIME_TO_EMPTY_AVG,
    TIME_TO_FULL_NOW,
    TIME_TO_FULL_AVG,
    TYPE, /* use power_supply.type instead */
    SCOPE,
    CHARGE_TERM_CURRENT,
    /* Properties of type `const char *' */
    MODEL_NAME,
    MANUFACTURER,
    SERIAL_NUMBER,
}

#[repr(C)]
pub enum power_supply_type {
    UNKNOWN = 0,
    BATTERY,
    UPS,
    MAINS,
    USB,      /* Standard Downstream Port */
    USB_DCP,  /* Dedicated Charging Port */
    USB_CDP,  /* Charging Downstream Port */
    USB_ACA,  /* Accessory Charger Adapters */
}

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
