.PHONY: all clean install

all:
	nasm -f elf32 start.asm -o start.o

clean:
	rm start.o

install:
	cp target/i686/debug/initializer $(CHIMERA_SRC_ROOT)/iso/boot/initializer
