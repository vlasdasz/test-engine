
include build/common.mk

ui3:
	cargo run -p ui_benchmark --profile=r3

bench:
	cargo run -p ui_benchmark --release
