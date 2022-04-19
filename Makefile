.PHONY: bridge run

## Homebrew installs LLVM in a place that is not visible to ffigen.
## This explicitly specifies the place where the LLVM dylibs are kept.
#llvm_path := if os() == "macos" {
#	"--llvm-path /opt/homebrew/opt/llvm"
#} else {
#	""
#}

default: bridge

lib/bridge_generated.dart: native/src/api.rs
	flutter_rust_bridge_codegen \
		--llvm-path=/usr \
		--llvm-compiler-opts="-I/usr/lib/clang/13.0.1/include" \
		--rust-input native/src/api.rs \
		--dart-output lib/bridge_generated.dart \
		--c-output ios/Runner/bridge_generated.h
	cp ios/Runner/bridge_generated.h macos/Runner/bridge_generated.h
	# Uncomment this line to invoke build_runner as well
	#flutter pub run build_runner build

bridge: lib/bridge_generated.dart

run: lib/bridge_generated.dart
	flutter run -d linux

clean:
	flutter clean
	cd native && cargo clean

