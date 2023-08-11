TARGET = aarch64-bare-metal
target := aarch64-unknown-none
mode := release
sysroot := $(shell rustc --print sysroot)
objdump := $(shell find $(sysroot) -name llvm-objdump) --arch-name=aarch64
objcopy := $(shell find $(sysroot) -name llvm-objcopy)

elf := target/$(target)/$(mode)/$(TARGET)
bin := target/$(target)/$(mode)/$(TARGET).bin

SOURCES = $(wildcard *.x) $(wildcard src/*.rs) \
	Cargo.toml

ifeq ($(mode), release)
	BUILD_ARGS += --release
endif

ifeq ($(DEBUG),1)
QEMU_ARGS = -S -s
endif

$(bin): $(elf)
	$(objcopy) -O binary $< $@

$(elf): $(SOURCES)
	cargo build $(BUILD_ARGS)

run: $(bin)
	qemu-system-aarch64 $(QEMU_ARGS) \
		-machine virt -cpu max \
		-nodefaults -nographic -nic none \
		-serial chardev:char0 \
		-chardev stdio,id=char0,mux=on \
		-trace *load* \
		-m 64 -smp 1 -kernel $(bin)

run-elf: $(elf)
	qemu-system-aarch64 $(QEMU_ARGS) \
		-machine virt -cpu max \
		-nodefaults -nographic -nic none \
		-serial chardev:char0 \
		-chardev stdio,id=char0,mux=on \
		-trace *load* \
		-m 64 -smp 1 -kernel $(elf)

debug: $(elf)
	aarch64-elf-gdb -ex "target remote 127.0.0.1:1234" \
		-ex "set disassemble-next-line on" $(elf)

dump: $(elf)
	$(objdump) -d $<
