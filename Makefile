KER = $(shell uname -r)
OBJ = vbat

RC = /usr/bin/rustc

obj-m = ${OBJ}.o
vbat-objs = stub.o main.o

all: ${OBJ}.ko

${OBJ}.ko: stub.c main.o
	make -C /lib/modules/$(KER)/build M=$(PWD) modules

%.o: %.rs
	$(RC) -O --crate-type lib -o $@ --emit obj $<
	./fixup $@

clean:
	make -C /lib/modules/$(KER)/build M=$(PWD) clean

insmod:
	sudo insmod $(OBJ).ko
	dmesg | tail

rmmod:
	sudo rmmod $(OBJ).ko
	dmesg | tail
