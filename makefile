.PHONY: dev setup-dev-guard

dev:
	chmod +x ./make/scripts/setup-guard-dev.sh
	./make/scripts/setup-guard-dev.sh

	cd server && cargo run

setup-dev-guard:
	chmod +x ./make/scripts/setup-guard-dev.sh
	./make/scripts/setup-guard-dev.sh