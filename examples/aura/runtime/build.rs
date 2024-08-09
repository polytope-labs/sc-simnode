fn main() {
	#[cfg(feature = "std")]
	{
		substrate_wasm_builder::WasmBuilder::build_using_defaults();
	}
}

/// The wasm builder is deactivated when compiling
/// this crate for wasm to speed up the compilation.
#[cfg(not(feature = "std"))]
fn main() {}
