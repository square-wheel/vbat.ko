KER = $(shell uname -r)
OBJ = vbat

obj-m = ${OBJ}.o
vbat-objs = stub.o

all:
	make -C /lib/modules/$(KER)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(KER)/build M=$(PWD) clean

insmod:
	sudo insmod $(OBJ).ko

rmmod:
	sudo rmmod $(OBJ).ko
