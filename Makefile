# edt-chess — convenience targets for the Build and Install Workflow

.PHONY: help test build release clean install package workflow check lint

PREFIX ?= $(HOME)/.local

help:
	@echo "edt-chess targets:"
	@echo "  make test       - run unit + integration tests"
	@echo "  make build      - debug build"
	@echo "  make release    - optimized release build"
	@echo "  make clean      - cargo clean"
	@echo "  make check      - cargo check"
	@echo "  make lint       - cargo check + deny warnings (rustc)"
	@echo "  make package    - full build-and-install workflow (tarball)"
	@echo "  make install    - package + install to PREFIX ($(PREFIX))"
	@echo "  make workflow   - alias for package"

test:
	cargo test

build:
	cargo build

release:
	cargo build --release

clean:
	cargo clean

check:
	cargo check

lint:
	RUSTFLAGS="-D warnings" cargo check
	RUSTFLAGS="-D warnings" cargo test --no-run

package workflow:
	./scripts/build-and-install.sh

install:
	./scripts/build-and-install.sh --install --prefix "$(PREFIX)"
