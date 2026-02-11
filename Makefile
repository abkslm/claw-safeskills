# Generic Makefile for Agent Skills
# Usage:
#   make          - Builds all skills
#   make <skill>  - Builds a specific skill (e.g., make nano-banana)
#   make clean    - Cleans all builds

# Find all subdirectories containing Cargo.toml
SKILLS_TOML := $(wildcard */Cargo.toml)
SKILLS := $(patsubst %/Cargo.toml,%,$(SKILLS_TOML))

.PHONY: all clean $(SKILLS)

all: $(SKILLS)

# Generic rule to build a skill
$(SKILLS):
	@echo "Building $@..."
	@cd $@ && cargo build --release
	@# Extract binary name from Cargo.toml
	@BINARY_NAME=$$(sed -n 's/^name = "\(.*\)"/\1/p' $@/Cargo.toml | head -n 1 | tr -d '\r'); \
	if [ -z "$$BINARY_NAME" ]; then \
		echo "Error: Could not determine binary name for $@"; \
		exit 1; \
	fi; \
	mkdir -p $@/scripts; \
	cp $@/target/release/$$BINARY_NAME $@/scripts/$$BINARY_NAME; \
	echo "Build complete. Binary copied to $@/scripts/$$BINARY_NAME"

clean:
	@for skill in $(SKILLS); do \
		echo "Cleaning $$skill..."; \
		(cd $$skill && cargo clean); \
		# Only remove binaries that match the Cargo.toml name to be safe
		BINARY_NAME=$$(sed -n 's/^name = "\(.*\)"/\1/p' $$skill/Cargo.toml | head -n 1 | tr -d '\r'); \
		if [ ! -z "$$BINARY_NAME" ]; then \
			rm -f $$skill/scripts/$$BINARY_NAME; \
		fi; \
	done
