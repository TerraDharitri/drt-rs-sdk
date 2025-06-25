numbat_wasm::imports!();
numbat_wasm::derive_imports!();

#[numbat_wasm::module]
pub trait EventsModule {
    #[event("buy-token")]
    fn buy_token_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);

    #[event("sell-token")]
    fn sell_token_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);
}
