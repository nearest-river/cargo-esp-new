TARGET=xtensa-esp8266-none-elf
export RUSTFLAGS=-C link-arg=-nostartfiles -C link-arg=-Wl,-Tlink.x


build:
	rustup run esp cargo build --target $(TARGET)

flash:
	rustup run esp cargo espflash flash --release --target $(TARGET)

run:
	rustup run esp cargo espflash flash --monitor --release --target $(TARGET)

