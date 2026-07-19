EXE := razor
VERSION := $(shell awk -F'"' '/^version/ {print $$2; exit}' Cargo.toml)
NAME := $(EXE)-$(VERSION)
OUT_DIR = bin

all:
	cargo build --release
	cp target/release/$(EXE) $(OUT_DIR)/$(NAME)
	@echo "Successfully staged $(NAME) into $(OUT_DIR)!"

clean:
	cargo clean
