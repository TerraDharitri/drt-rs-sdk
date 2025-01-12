dharitri_sc::imports!();

/// Used for testing the moa_decimal function return type
#[dharitri_sc::module]
pub trait MoaDecimal {
    #[payable("MOA")]
    #[endpoint]
    fn returns_moa_decimal(&self) -> ManagedDecimal<Self::Api, ConstDecimals<18>> {
        self.call_value().moa_decimal()
    }
}
