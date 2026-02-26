#![no_std]

numbat_wasm::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[numbat_wasm::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}
}
