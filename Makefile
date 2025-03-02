.SILENT:

# export PROJECT_NAME := ferrisboy
export PROJECT_ROOT := $(shell pwd)
export SCRIPTS_DIR := $(PROJECT_ROOT)/scripts
MAKEFILE_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

.DEFAULT_GOAL := help

.PHONY: help
h help: ## Display this help message
	@echo "\033[33mUSAGE: make [TARGET]\033[0m"
	@echo ""
	@echo "\033[33mTARGET:\033[0m"
	@grep -hE '^[ a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\t\033[36m%-15s\033[0m %s\n", $$1, $$2}'

.PHONY: deps
d deps: ## Install project dependencies
	@echo "Linking pre-push git hook..."
	cd $(MAKEFILE_DIR) && \
		ln -sf $(SCRIPTS_DIR)/git-pre-push.sh .git/hooks/pre-push
	@echo "Linked pre-push git hook"

.PHONY: lint
l lint:  ## Lint
	$(SCRIPTS_DIR)/lint.sh

.PHONY: build
b build: ## Build project
	@echo "Use \`cargo\` to build project"
