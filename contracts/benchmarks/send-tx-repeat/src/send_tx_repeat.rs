#![no_std]

numbat_wasm::imports!();

#[numbat_wasm::contract]
pub trait SendTxRepeat {
    #[init]
    fn init(&self) {}

    #[payable("REWA")]
    #[endpoint]
    fn repeat(&self, to: ManagedAddress, amount: BigUint, times: usize) {
        for _ in 0..times {
            self.send().direct_rewa(&to, &amount);
        }
    }
}
