#![no_std]

numbat_wasm::imports!();

#[numbat_wasm::contract]
pub trait SendTxRepeat {
    #[init]
    fn init(&self) {}

    #[payable("REWA")]
    #[endpoint]
    fn repeat(
        &self,
        to: ManagedAddress,
        amount: BigUint,
        times: usize,
        #[var_args] opt_data: OptionalValue<BoxedBytes>,
    ) {
        let data = match opt_data {
            OptionalValue::Some(d) => d,
            OptionalValue::None => BoxedBytes::empty(),
        };
        for _ in 0..times {
            self.send().direct_rewa(&to, &amount, data.as_slice());
        }
    }
}
