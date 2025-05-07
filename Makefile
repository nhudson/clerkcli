VERSION  := v0.1.0
REVISION := $(shell git rev-parse --short HEAD)
REGISTRY ?= loocalhost:5001

############################################################################
# Build

# Build the project (debug)
PHONY: build
build:
	cargo build

# Build the project (release)
PHONY: release
release:
	cargo build --release

# Run the binary (debug)
PHONY: run
run:
	cargo run --

# Run tests
PHONY: test
test:
	cargo test

# Run clippy linter
PHONY: clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
PHONY: fmt
fmt:
	cargo fmt --all -- --check 

############################################################################
# OCI Images

PHONY: image
image:
	registry=$(REGISTRY) version=$(VERSION) revision=$(REVISION) docker buildx bake $(if $(filter true,$(PUSH)),--push,)

