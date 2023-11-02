
include build/common.mk

ui:
	cargo run -p ui_test && cargo run -p ui_test --release
