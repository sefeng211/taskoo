# Makefile

build-cli:
	cargo build -p taskoo-cli

build-core:
	cargo build -p taskoo-core

build-web:
	cd taskoo-core && bash build_wasm.sh && cd .. && \
	cp target/wasm32-wasip1/release/taskoo_core.wasm taskoo-web/server/taskoo_core.wasm

run-web-backend:
	cd taskoo-web/server && npm install && npm run start

run-web-ui:
	cd taskoo-web/webpack-ver && npm install && npm run start
