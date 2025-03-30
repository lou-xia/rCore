TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
BUILD_DIR := build
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
PY := python3

TEST ?= 0

# INFO is my test for fn sys_task_info()
INFO ?= 0
ifeq ($(TEST), 0)
	ifeq ($(INFO), 1)
		APPS := $(wildcard $(APP_DIR)/yield*.rs) \
			$(wildcard $(APP_DIR)/power*.rs) \
            $(wildcard $(APP_DIR)/z*.rs)
	else if ($(INFO), 2)
		APPS := $(wildcard $(APP_DIR)/yield*.rs)
	else
		APPS :=  $(filter-out $(wildcard $(APP_DIR)/test*.rs), $(wildcard $(APP_DIR)/*.rs))
	endif
else	
		APPS :=  $(wildcard $(APP_DIR)/test$(TEST)*.rs)
endif
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

elf: $(APPS)
	@python3 build.py

binary: elf
	@$(foreach elf, $(ELFS), \
		$(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf)); \
		cp $(elf) $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.elf, $(elf));)

pre:
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/
	@mkdir -p $(BUILD_DIR)/app/
	@$(foreach t, $(APPS), cp $(t) $(BUILD_DIR)/app/;)
	
build: clean pre binary 
	@$(foreach t, $(ELFS), cp $(t).bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(ELFS), cp $(t).elf $(BUILD_DIR)/elf/;)

clean:
	@cargo clean
	@rm -rf $(BUILD_DIR)

.PHONY: elf binary build clean