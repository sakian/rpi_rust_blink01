RUSTC=rustc
TARGET_FLAGS=--target arm-unknown-linux-gnueabihf
RUST_FLAGS=-O --emit=obj
GCC=arm-none-eabi-gcc
CFLAGS=-O0 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostartfiles --specs=nosys.specs
OBJ_COPY=arm-none-eabi-objcopy
SHARE_DIR=/home/mark/share/proj

all: kernel.img install

kernel.img: kernel.elf
	${OBJ_COPY} $< -O binary $@

kernel.elf: kernel.o
	${GCC} ${CFLAGS} $< -o $@

kernel.o: kernel.rs
	${RUSTC} ${TARGET_FLAGS} ${RUST_FLAGS} $<

install: kernel.img
	mkdir -p ${SHARE_DIR}/blinky
	cp $< ${SHARE_DIR}/blinky

clean:
	rm *.o *.elf *.img

