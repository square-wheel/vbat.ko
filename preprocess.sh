#!/bin/sh
gcc -E /usr/src/linux/include/linux/power_supply.h -I /usr/src/linux/include -I /usr/src/linux/arch/x86/include/ -include /usr/src/linux/include/generated/autoconf.h -o power_supply.i
