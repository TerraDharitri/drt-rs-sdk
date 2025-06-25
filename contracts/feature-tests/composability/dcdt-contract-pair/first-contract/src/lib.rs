#![no_std]

numbat_wasm::imports!();

const DCDT_TRANSFER_STRING: &[u8] = b"DCDTTransfer";
const SECOND_CONTRACT_ACCEPT_DCDT_PAYMENT: &[u8] = b"acceptDcdtPayment";
const SECOND_CONTRACT_REJECT_DCDT_PAYMENT: &[u8] = b"rejectDcdtPayment";

#[numbat_wasm::contract]
pub trait FirstContract {
    #[init]
    fn init(
        &self,
        dcdt_token_identifier: TokenIdentifier,
        second_contract_address: ManagedAddress,
    ) {
        self.set_contract_dcdt_token_identifier(&dcdt_token_identifier);
        self.set_second_contract_address(&second_contract_address);
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractFull)]
    fn transfer_to_second_contract_full(
        &self,
        #[payment] dcdt_value: BigUint,
        #[payment_token] actual_token_identifier: TokenIdentifier,
    ) {
        let expected_token_identifier = self.get_contract_dcdt_token_identifier();

        require!(dcdt_value > 0, "no dcdt transfered!");
        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dcdt token"
        );

        self.call_dcdt_second_contract(
            &expected_token_identifier,
            &dcdt_value,
            &self.get_second_contract_address(),
            &ManagedBuffer::from(SECOND_CONTRACT_ACCEPT_DCDT_PAYMENT),
            &ManagedVec::new(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractHalf)]
    fn transfer_to_second_contract_half(
        &self,
        #[payment] dcdt_value: BigUint,
        #[payment_token] actual_token_identifier: TokenIdentifier,
    ) {
        let expected_token_identifier = self.get_contract_dcdt_token_identifier();

        require!(dcdt_value > 0, "no dcdt transfered!");
        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dcdt token"
        );

        self.call_dcdt_second_contract(
            &expected_token_identifier,
            &(dcdt_value / 2u32),
            &self.get_second_contract_address(),
            &ManagedBuffer::from(SECOND_CONTRACT_ACCEPT_DCDT_PAYMENT),
            &ManagedVec::new(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractRejected)]
    fn transfer_to_second_contract_rejected(
        &self,
        #[payment] dcdt_value: BigUint,
        #[payment_token] actual_token_identifier: TokenIdentifier,
    ) {
        let expected_token_identifier = self.get_contract_dcdt_token_identifier();

        require!(dcdt_value > 0, "no dcdt transfered!");
        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dcdt token"
        );

        self.call_dcdt_second_contract(
            &expected_token_identifier,
            &dcdt_value,
            &self.get_second_contract_address(),
            &ManagedBuffer::from(SECOND_CONTRACT_REJECT_DCDT_PAYMENT),
            &ManagedVec::new(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractRejectedWithTransferAndExecute)]
    fn transfer_to_second_contract_rejected_with_transfer_and_execute(
        &self,
        #[payment] dcdt_value: BigUint,
        #[payment_token] actual_token_identifier: TokenIdentifier,
    ) {
        let second_contract_address = self.get_second_contract_address();
        let expected_token_identifier = self.get_contract_dcdt_token_identifier();

        require!(dcdt_value > 0u32, "no dcdt transfered!");
        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dcdt token"
        );

        let _ = Self::Api::send_api_impl().direct_dcdt_execute(
            &second_contract_address,
            &expected_token_identifier,
            &dcdt_value,
            self.blockchain().get_gas_left(),
            &ManagedBuffer::from(SECOND_CONTRACT_REJECT_DCDT_PAYMENT),
            &ManagedArgBuffer::new_empty(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractFullWithTransferAndExecute)]
    fn transfer_to_second_contract_full_with_transfer_and_execute(
        &self,
        #[payment] dcdt_value: BigUint,
        #[payment_token] actual_token_identifier: TokenIdentifier,
    ) {
        let second_contract_address = self.get_second_contract_address();
        let expected_token_identifier = self.get_contract_dcdt_token_identifier();

        require!(dcdt_value > 0u32, "no dcdt transfered!");
        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dcdt token"
        );

        let _ = Self::Api::send_api_impl().direct_dcdt_execute(
            &second_contract_address,
            &expected_token_identifier,
            &dcdt_value,
            self.blockchain().get_gas_left(),
            &ManagedBuffer::from(SECOND_CONTRACT_ACCEPT_DCDT_PAYMENT),
            &ManagedArgBuffer::new_empty(),
        );
    }

    fn call_dcdt_second_contract(
        &self,
        dcdt_token_identifier: &TokenIdentifier,
        amount: &BigUint,
        to: &ManagedAddress,
        func_name: &ManagedBuffer,
        args: &ManagedVec<Self::Api, ManagedBuffer>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new_empty();
        arg_buffer.push_arg(dcdt_token_identifier);
        arg_buffer.push_arg(amount);
        arg_buffer.push_arg(func_name);
        for arg in args.into_iter() {
            arg_buffer.push_arg_raw(arg);
        }

        Self::Api::send_api_impl().async_call_raw(
            to,
            &BigUint::zero(),
            &ManagedBuffer::from(DCDT_TRANSFER_STRING),
            &arg_buffer,
        );
    }

    // storage

    #[storage_set("dcdtTokenName")]
    fn set_contract_dcdt_token_identifier(&self, dcdt_token_identifier: &TokenIdentifier);

    #[view(getdcdtTokenName)]
    #[storage_get("dcdtTokenName")]
    fn get_contract_dcdt_token_identifier(&self) -> TokenIdentifier;

    #[storage_set("secondContractAddress")]
    fn set_second_contract_address(&self, address: &ManagedAddress);

    #[view(getSecondContractAddress)]
    #[storage_get("secondContractAddress")]
    fn get_second_contract_address(&self) -> ManagedAddress;
}
