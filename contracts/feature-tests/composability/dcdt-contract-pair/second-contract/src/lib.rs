#![no_std]

numbat_wasm::imports!();

#[numbat_wasm::contract]
pub trait SecondContract {
    #[init]
    fn init(&self, dcdt_token_identifier: TokenIdentifier) {
        self.set_contract_dcdt_token_identifier(&dcdt_token_identifier);
    }

    #[payable("*")]
    #[endpoint(acceptDcdtPayment)]
    fn accept_dcdt_payment(&self, #[payment_token] actual_token_identifier: TokenIdentifier) {
        let expected_token_identifier = self.get_contract_dcdt_token_identifier();
        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dcdt token"
        );
    }

    #[payable("*")]
    #[endpoint(rejectDcdtPayment)]
    fn reject_dcdt_payment(&self) {
        sc_panic!("Rejected")
    }

    // storage

    #[storage_set("dcdtTokenName")]
    fn set_contract_dcdt_token_identifier(&self, dcdt_token_identifier: &TokenIdentifier);

    #[view(getdcdtTokenName)]
    #[storage_get("dcdtTokenName")]
    fn get_contract_dcdt_token_identifier(&self) -> TokenIdentifier;
}
