.PHONY: help debug release dist clean deps coconut

.DEFAULT_GOAL := help

REPO_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/..)
CMAKE ?= cmake
CMAKE_ARGS ?=

help:
	@echo "Targets:"
	@echo "  debug  : Build CoCoNut and a debug compiler in build/compiler"
	@echo "  release: Build CoCoNut and a release compiler in build/compiler"
	@echo "  coconut: Build bundled CoCoNut in build/coconut"
	@echo "  deps   : Alias for coconut; system prerequisites must already be installed"
	@echo "  dist   : Create a submission archive in build/dist/civicc.tgz"
	@echo "  clean  : Remove build/compiler, build/coconut, and build/dist"
	@echo "Debug and release reuse build/compiler. Pass CMAKE_ARGS for extra CMake options."

coconut:
	$(CMAKE) -S "$(REPO_DIR)/vendor/coconut" -B "$(REPO_DIR)/build/coconut" -DCMAKE_BUILD_TYPE=Release
	$(CMAKE) --build "$(REPO_DIR)/build/coconut"

deps: coconut

# For example: make debug CMAKE_ARGS=-DDISABLE_ASAN=ON
debug: coconut
	$(CMAKE) -S "$(REPO_DIR)/compiler" -B "$(REPO_DIR)/build/compiler" -DCMAKE_BUILD_TYPE=Debug $(CMAKE_ARGS)
	$(CMAKE) --build "$(REPO_DIR)/build/compiler"

release: coconut
	$(CMAKE) -S "$(REPO_DIR)/compiler" -B "$(REPO_DIR)/build/compiler" -DCMAKE_BUILD_TYPE=Release $(CMAKE_ARGS)
	$(CMAKE) --build "$(REPO_DIR)/build/compiler"

dist:
	bash "$(REPO_DIR)/compiler/scripts/dist.sh"

clean:
	rm -rf -- "$(REPO_DIR)/build/compiler" "$(REPO_DIR)/build/coconut" "$(REPO_DIR)/build/dist"
