# Zig Testing Makefile
# Modern Zig testing tooling

.PHONY: test test-all test-cover test-fuzz test-doc test-ci

# Zig commands
ZIG := zig
ZIGBUILD := $(ZIG) build
ZIGTEST := $(ZIG) test

# Version
ZIG_VERSION := $(shell $(ZIG) version)

# Test targets
test: ## Run all tests
	$(ZIGTEST) .

test-lib: ## Run library tests
	$(ZIGTEST) --lib .

test-example: ## Run example tests
	$(ZIGTEST) --mod=examples .

test-cover: ## Run tests with coverage
	$(ZIGTEST) . --enable-coverage
	mkdir -p coverage
	$(ZIG) cov html coverage/

test-fuzz: ## Run fuzzing tests (short)
	$(ZIGTEST) . --fuzz

test-fuzz-long: ## Run fuzzing tests (extended)
	$(ZIGTEST) . --fuzz --fuzz-timeout=60s

# Benchmark
test-bench: ## Run benchmarks
	$(ZIG) build bench

# Doc tests
test-doc: ## Run doc tests
	$(ZIG) test --doc-on-lib ..

# CI target
test-ci: test test-fuzz test-doc
	@echo "All tests passed on Zig $(ZIG_VERSION)"

# Format
fmt: ## Format code
	$(ZIG) fmt .
	$(ZIG) fmt --check .

# Lint (experimental)
lint: ## Run linter
	$(ZIG) fmt --check . || true

# Clean
clean-test:
	rm -rf coverage zig-out/*.asm
	find . -name '*.gcda' -delete
	find . -name '*.gcno' -delete
	find . -name '*.profraw' -delete

## help: Show this help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
