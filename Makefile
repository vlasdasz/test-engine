
all: desktop

desktop:
	./build/build.sh

ios:
	./build/build.sh ios

lipo:
	cargo lipo

android:
	./build/build.sh android

clippy:
	cargo clippy -- -D clippy::pedantic -A clippy::module-name-repetitions -A clippy::must_use_candidate -A clippy::implicit-hasher -A clippy::missing_errors_doc -A clippy::semicolon_if_nothing_returned -A clippy::return_self_not_must_use -A clippy::default_trait_access -A clippy::needless_pass_by_value -A clippy::missing_panics_doc -A clippy::mismatched_target_os -A clippy::explicit-deref-methods -A clippy::cast-precision-loss -A clippy::module_inception
