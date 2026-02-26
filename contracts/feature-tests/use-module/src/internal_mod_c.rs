numbat_wasm::imports!();

/// Example of a module that lies in the same crate.
#[numbat_wasm::module]
pub trait InternalModuleC {
    #[view]
    fn call_mod_c(&self) {}
}
