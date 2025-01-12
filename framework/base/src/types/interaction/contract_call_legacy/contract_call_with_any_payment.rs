use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{MoaOrMultiDcdtPayment, ManagedAddress, ManagedBuffer},
};

use super::{
    contract_call_no_payment::ContractCallNoPayment, contract_call_trait::ContractCallBase,
    ContractCall, ContractCallWithMoa,
};

/// Holds data for calling another contract, with any type of payment: none, MOA, Multi-DCDT.
///
/// Gets created when chaining method `with_any_payment`.
#[deprecated(
    since = "0.49.0",
    note = "Please use the unified transaction syntax instead."
)]
#[must_use]
pub struct ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub payment: MoaOrMultiDcdtPayment<SA>,
}

impl<SA, OriginalResult> ContractCallBase<SA> for ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithMoa<SA, Self::OriginalResult> {
        match self.payment {
            MoaOrMultiDcdtPayment::Moa(moa_amount) => self.basic.with_moa_transfer(moa_amount),
            MoaOrMultiDcdtPayment::MultiDcdt(multi_dcdt_payment) => self
                .basic
                .into_normalized()
                .convert_to_dcdt_transfer_call(multi_dcdt_payment),
        }
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        match self.payment {
            MoaOrMultiDcdtPayment::Moa(moa_amount) => {
                self.basic.transfer_execute_moa(moa_amount);
            },
            MoaOrMultiDcdtPayment::MultiDcdt(multi_dcdt_payment) => {
                self.basic.transfer_execute_dcdt(multi_dcdt_payment);
            },
        }
    }
}

impl<SA, OriginalResult> ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// Creates a new instance directly.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        payment: MoaOrMultiDcdtPayment<SA>,
    ) -> Self {
        ContractCallWithAnyPayment {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            payment,
        }
    }
}
