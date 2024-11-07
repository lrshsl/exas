SRC_DIR := src
BUILD_DIR := build

ASM_FILES := $(patsubst $(SRC_DIR)/%.asm,%,$(wildcard $(SRC_DIR)/*.asm))
ld := ld
ldflags := -lc -dynamic-linker /lib64/ld-linux-x86-64.so.2

all: $(ASM_FILES:%=$(BUILD_DIR)/%) $(ASM_FILES:%=$(BUILD_DIR)/%.lst)

$(BUILD_DIR)/%: $(SRC_DIR)/%.asm
	@mkdir -p $(BUILD_DIR)
	nasm -o $(BUILD_DIR)/$*.o -f elf64 $< -l $(BUILD_DIR)/$*.lst
	$(ld) $(ldflags) -o $@ $(BUILD_DIR)/$*.o
	@rm $(BUILD_DIR)/*.o
	@echo

clean:
	@rm -rf $(BUILD_DIR)

