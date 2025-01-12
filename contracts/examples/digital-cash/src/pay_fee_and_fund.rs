use dharitri_sc::imports::*;

use crate::{constants::*, helpers, storage};

#[dharitri_sc::module]
pub trait PayFeeAndFund: storage::StorageModule + helpers::HelpersModule {
    #[endpoint(payFeeAndFundDCDT)]
    #[payable]
    fn pay_fee_and_fund_dcdt(&self, address: ManagedAddress, valability: u64) {
        let mut payments = self.call_value().all_dcdt_transfers().clone();
        let fee = MoaOrDcdtTokenPayment::from(payments.get(0).clone());
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee);

        payments.remove(0);

        self.make_fund(0u64.into(), payments, address, valability)
    }
    #[endpoint(payFeeAndFundMOA)]
    #[payable("MOA")]
    fn pay_fee_and_fund_moa(&self, address: ManagedAddress, valability: u64) {
        let mut fund = self.call_value().moa().clone();
        let fee_value = self.fee(&MoaOrDcdtTokenIdentifier::moa()).get();
        require!(fund > fee_value, "payment not covering fees");

        fund -= fee_value.clone();
        let fee = MoaOrDcdtTokenPayment::new(MoaOrDcdtTokenIdentifier::moa(), 0, fee_value);
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee);

        self.make_fund(fund, ManagedVec::new(), address, valability);
    }

    #[endpoint]
    #[payable]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        require!(!self.deposit(&address).is_empty(), FEES_NOT_COVERED_ERR_MSG);
        let deposit_mapper = self.deposit(&address).get();
        let depositor = deposit_mapper.depositor_address;
        require!(
            self.blockchain().get_caller() == depositor,
            "invalid depositor"
        );
        let deposited_fee_token = deposit_mapper.fees.value;
        let fee_amount = self.fee(&deposited_fee_token.token_identifier).get();
        // TODO: switch to moa+dcdt multi transfer handling
        let moa_payment = self.call_value().moa_direct_non_strict().clone();
        let dcdt_payment = self.call_value().all_dcdt_transfers().clone();

        let num_tokens = self.get_num_token_transfers(&moa_payment, &dcdt_payment);
        self.check_fees_cover_number_of_tokens(num_tokens, fee_amount, deposited_fee_token.amount);

        self.make_fund(moa_payment, dcdt_payment, address, valability);
    }

    #[endpoint(depositFees)]
    #[payable("MOA")]
    fn deposit_fees(&self, address: &ManagedAddress) {
        let payment = self.call_value().moa_or_single_dcdt();
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, address, payment);
    }
}
