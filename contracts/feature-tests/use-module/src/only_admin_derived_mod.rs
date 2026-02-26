numbat_wasm::imports!();

#[numbat_wasm::module]
pub trait OnlyAdminDerivedTestModule {
    #[view]
    fn call_derived_not_admin_only(&self) {}
}
