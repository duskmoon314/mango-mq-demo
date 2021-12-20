# TOOLS
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
XFEL := ~/Documents/programs/xfel/xfel

# BUILD CONFIG
TARGET := riscv64gc-unknown-none-elf
MODE ?= debug
APP_NAME := blink
APP_ELF := target/${TARGET}/${MODE}/${APP_NAME}
APP_BIN := ${APP_ELF}.bin

all: run

env:
	@(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	@cargo install cargo-binutils
	@rustup component add rust-src
	@rustup component add llvm-tools-preview

build: env ${APP_BIN}

${APP_BIN}: app
	@${OBJCOPY} ${APP_ELF} --strip-all -O binary $@

app:
ifeq (${MODE}, debug)
	@cargo build
else
	@cargo build --release
endif

flash: build
	@${XFEL} ddr ddr2
	@${XFEL} write 0x80000000 ${APP_BIN}

run: flash
	@${XFEL} exec 0x80000000

clean:
	@cargo clean

.PHONY: env build app flash run clean