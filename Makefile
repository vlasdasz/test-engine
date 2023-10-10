
include build/common.mk

test:
	python3 test.py

test_ui:
	cargo run -p ui_test
