.PHONY: flash flashm build dump_section_size dump_flash_rodata dump_flash_text
flash:
	ESP_IDF_SYS_ROOT_CRATE=esp cargo espflash flash --release -p /dev/ttyUSB0 -f 80mhz -b 921600
flashm:
	ESP_IDF_SYS_ROOT_CRATE=esp cargo espflash flash --release -p /dev/ttyUSB0 -f 80mhz -b 921600 -M
build:
	ESP_IDF_SYS_ROOT_CRATE=esp cargo build --release
dump_section_size:
	readelf --demangle -S ../target/xtensa-esp32-espidf/release/matrix-fire  | grep -oP '\..*$' | sort -k 5 | grep -v debug | grep -vE 'TAB|\.xt\.' | tail

dump_flash_text:
	 objdump --demangle -t -j .flash.text ../target/xtensa-esp32-espidf/release/matrix-fire | sort -k 5 | grep -v '00000000' | tail

dump_flash_rodata:
	 objdump --demangle -t -j .flash.rodata ../target/xtensa-esp32-espidf/release/matrix-fire | sort -k 5 | grep -v '00000000' | tail
