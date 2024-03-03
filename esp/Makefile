.PHONY: flash flashm build
flash:
	ESP_IDF_SYS_ROOT_CRATE=esp cargo espflash flash --release -p /dev/ttyUSB0 -f 80mhz -b 921600
flashm:
	ESP_IDF_SYS_ROOT_CRATE=esp cargo espflash flash --release -p /dev/ttyUSB0 -f 80mhz -b 921600 -M
build:
	ESP_IDF_SYS_ROOT_CRATE=esp cargo build --release
