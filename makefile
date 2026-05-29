# Lighthouse dev environment.
#
# Quick start:
#   make dev            Start the API server and frontend together.
#
# Override ports/endpoint on the command line, e.g.:
#   make dev API_PORT=5000
#   make dev-frontend API_ENDPOINT=https://staging.example.com

# --- Config -----------------------------------------------------------------
API_PORT     ?= 4459
API_ENDPOINT ?= http://localhost:$(API_PORT)
FRONTEND_DIR ?= frontend/lighthouse

.DEFAULT_GOAL := help
.PHONY: help dev dev-server dev-frontend setup-guard setup-dev-guard

help: ## Show this help
	@echo "Lighthouse make targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Config (override with VAR=value): API_PORT=$(API_PORT) API_ENDPOINT=$(API_ENDPOINT)"

dev: setup-guard ## Run the API server and frontend together (Ctrl-C stops both)
	@chmod +x ./make/scripts/dev.sh
	@API_PORT="$(API_PORT)" API_ENDPOINT="$(API_ENDPOINT)" FRONTEND_DIR="$(FRONTEND_DIR)" \
		./make/scripts/dev.sh

dev-server: setup-guard ## Run only the Rust API server (on API_PORT)
	cd server && lighthouse_port=$(API_PORT) cargo run

dev-frontend: ## Run only the Next.js frontend (pointed at API_ENDPOINT)
	cd $(FRONTEND_DIR) && NEXT_PUBLIC_api_endpoint="$(API_ENDPOINT)" yarn dev

setup-guard: ## Download the local Guard binary if missing
	@chmod +x ./make/scripts/setup-guard-dev.sh
	@./make/scripts/setup-guard-dev.sh

# Backwards-compatible alias for the previous target name.
setup-dev-guard: setup-guard
