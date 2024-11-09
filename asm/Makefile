src_dir := src
build_dir := build

asm_files := $(patsubst $(src_dir)/%.asm,%,$(wildcard $(src_dir)/*.asm))
ld := ld
ldflags := -lc -dynamic-linker /lib64/ld-linux-x86-64.so.2

all: $(asm_files:%=$(build_dir)/%) $(asm_files:%=$(build_dir)/%.lst)

$(build_dir)/%: $(src_dir)/%.asm
	@mkdir -p $(build_dir)
	nasm -o $(build_dir)/$*.o -f elf64 $< -l $(build_dir)/$*.lst
	$(ld) $(ldflags) -o $@ $(build_dir)/$*.o
	@rm $(build_dir)/*.o
	@echo

run: $(build_dir)/fileaccess_macros
	./$<
	cat file.txt

clean:
	@rm -rf $(build_dir)

