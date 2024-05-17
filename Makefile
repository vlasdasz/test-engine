
include build/common.mk

ui3:
	cargo run -p ui_benchmark --profile=r3

fix:
	cargo fix --allow-dirty --allow-staged --all

bench:
	cargo run -p ui_benchmark --release
