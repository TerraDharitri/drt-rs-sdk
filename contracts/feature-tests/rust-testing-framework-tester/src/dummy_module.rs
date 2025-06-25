numbat_wasm::imports!();

#[numbat_wasm::module]
pub trait DummyModule {
    fn some_function(&self) -> BigUint {
        BigUint::zero()
    }
}
