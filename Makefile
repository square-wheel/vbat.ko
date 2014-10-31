KER = $(shell uname -r)
OBJ = stub

obj-m = ${OBJ}.o

all:
	make -C /lib/modules/$(KER)/build M=$(PWD) modules
