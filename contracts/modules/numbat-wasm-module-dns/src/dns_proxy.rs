numbat_wasm::imports!();

#[numbat_wasm::proxy]
pub trait Dns {
    #[payable("REWA")]
    #[endpoint]
    fn register(&self, name: BoxedBytes, #[payment] payment: Self::BigUint);
}
