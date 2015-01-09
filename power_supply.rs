#![allow(dead_code)]

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
    VOLTAGE_BOOT,
    CURRENT_MAX,
    CURRENT_NOW,
    CURRENT_AVG,
    CURRENT_BOOT,
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
    CALIBRATE,
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
