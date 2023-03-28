export CPATH := `clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d' ' -f1 | rev` / 'include'

default: gen lint

gen:
	flutter pub get
	flutter_rust_bridge_codegen \
		--rust-input rust/src/api.rs \
		--dart-output lib/bridge_generated.dart \
		--dart-decl-output lib/bridge_definitions.dart

gen_apple:
	flutter pub get
	flutter_rust_bridge_codegen \
		--rust-input rust/src/api.rs \
		--dart-output lib/bridge_generated.dart \
		--c-output ios/Runner/bridge_generated.h \
		--dart-decl-output lib/bridge_definitions.dart
	cp ios/Runner/bridge_generated.h macos/Runner/bridge_generated.h

lint:
	cd rust && cargo fmt
	dart format .

clean:
	flutter clean
	cd rust && cargo clean

run *args='': gen
	flutter run {{args}}

run_apple *args='': gen_apple
	flutter run {{args}}
