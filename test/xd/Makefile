
TARGET=xtensa-esp8266-none-elf


build:
  rustup run esp cargo build --target $(TARGET)
flash:
  rustup run esp cargo espflash flash --release --target $(TARGET)
run:
  rustup run esp cargo run --release --target $(TARGET)
