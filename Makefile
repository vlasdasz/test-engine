
include build/common.mk

render:
	cargo run -p render-test

ui:
	cargo run -p ui-test
	cargo run -p ui-test --release

all:
	order
	make ios
	make ui
	make render

ui3:
	cargo run -p ui-benchmark --profile=r3

fix:
	cargo fix --allow-dirty --allow-staged --all

bench:
	cargo run -p ui-benchmark --release

mobile:
	cargo install test-mobile
	test-mobile --path=../test-mobile/mobile-template

ios-lib:
	cargo lipo -p test-game --release

ios-debug:
	cargo lipo -p test-game
	rm -f ./target/universal/release/libtest_game.a
	cp ./target/universal/debug/libtest_game.a ./target/universal/release/libtest_game.a


lint:
	cargo clippy \
      -- \
      \
      -W clippy::all \
      -W clippy::pedantic \
      \
      -A clippy::missing_panics_doc \
      -A clippy::module_name_repetitions \
      -A clippy::explicit_deref_methods \
      -A clippy::missing_errors_doc \
      -A clippy::must_use_candidate \
      -A clippy::module_inception \
      -A clippy::needless_pass_by_value \
      -A clippy::unnecessary_box_returns \
      -A clippy::return_self_not_must_use \
      -A clippy::struct_field_names \
      -A dead_code \
      \
      -D warnings

serve:
	rustup target add wasm32-unknown-unknown
	cargo install --locked trunk
	cd ./test-game && trunk serve --features webgl

serve-r:
	rustup target add wasm32-unknown-unknown
	cargo install --locked trunk
	cd ./test-game && trunk serve --features webgl --release

.PHONY: mobile
