use core::marker::PhantomData;

use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, MoaOrDcdtTokenIdentifier, MoaOrDcdtTokenPayment, MoaOrMultiDcdtPayment,
        DcdtTokenPayment, FunctionCall, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
        ManagedVec, TokenIdentifier, Tx, TxScEnv,
    },
};

use super::{
    contract_call_trait::ContractCallBase, contract_call_with_moa::ContractCallWithMoa,
    contract_call_with_multi_dcdt::ContractCallWithMultiDcdt, ContractCall,
    ContractCallWithAnyPayment, ContractCallWithMoaOrSingleDcdt, UNSPECIFIED_GAS_LIMIT,
};

/// Holds metadata for calling another contract, without payments.
///
/// Proxies generally create contract calls of this type
/// (unless there are payment arguments in the endpoint - but these are mostly obsolete now).
///
/// It is also the basis for all other contract call types, all of them contain this one.
#[deprecated(
    since = "0.49.0",
    note = "Please use the unified transaction syntax instead."
)]
#[must_use]
pub struct ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(crate) _phantom: PhantomData<SA>,
    pub to: ManagedAddress<SA>,
    pub function_call: FunctionCall<SA>,
    pub explicit_gas_limit: u64,
    pub(crate) _return_type: PhantomData<OriginalResult>,
}

impl<SA, OriginalResult> ContractCallBase<SA> for ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    #[inline]
    fn into_normalized(self) -> ContractCallWithMoa<SA, Self::OriginalResult> {
        ContractCallWithMoa {
            basic: self,
            moa_payment: BigUint::zero(),
        }
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        self
    }

    fn transfer_execute(self) {
        self.transfer_execute_moa(BigUint::zero());
    }
}

impl<SA, OriginalResult> ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn new<N: Into<ManagedBuffer<SA>>>(to: ManagedAddress<SA>, endpoint_name: N) -> Self {
        ContractCallNoPayment {
            _phantom: PhantomData,
            to,
            function_call: FunctionCall {
                function_name: endpoint_name.into(),
                arg_buffer: ManagedArgBuffer::new(),
            },
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            _return_type: PhantomData,
        }
    }

    /// Sets payment to be MOA transfer.
    pub fn with_moa_transfer(
        self,
        moa_amount: BigUint<SA>,
    ) -> ContractCallWithMoa<SA, OriginalResult> {
        ContractCallWithMoa {
            basic: self,
            moa_payment: moa_amount,
        }
    }

    /// Adds a single DCDT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_dcdt_transfer<P: Into<DcdtTokenPayment<SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithMultiDcdt<SA, OriginalResult> {
        let result = ContractCallWithMultiDcdt {
            basic: self,
            dcdt_payments: ManagedVec::new(),
        };
        result.with_dcdt_transfer(payment)
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_dcdt_transfer((payment_token, payment_nonce, payment_amount))`. 
        The tuple argument will get automatically converted to DcdtTokenPayment."
    )]
    pub fn add_dcdt_token_transfer(
        self,
        payment_token: TokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> ContractCallWithMultiDcdt<SA, OriginalResult> {
        self.with_dcdt_transfer((payment_token, payment_nonce, payment_amount))
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_multi_token_transfer(
        self,
        payments: ManagedVec<SA, DcdtTokenPayment<SA>>,
    ) -> ContractCallWithMultiDcdt<SA, OriginalResult> {
        ContractCallWithMultiDcdt {
            basic: self,
            dcdt_payments: payments,
        }
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_any_payment(
        self,
        payment: MoaOrMultiDcdtPayment<SA>,
    ) -> ContractCallWithAnyPayment<SA, OriginalResult> {
        ContractCallWithAnyPayment {
            basic: self,
            payment,
        }
    }

    /// Sets payment to be either MOA or a single DCDT transfer, as determined at runtime.
    pub fn with_moa_or_single_dcdt_transfer<P: Into<MoaOrDcdtTokenPayment<SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithMoaOrSingleDcdt<SA, OriginalResult> {
        ContractCallWithMoaOrSingleDcdt {
            basic: self,
            payment: payment.into(),
        }
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_moa_or_single_dcdt_transfer((payment_token, payment_nonce, payment_amount))`. "
    )]
    pub fn with_moa_or_single_dcdt_token_transfer(
        self,
        payment_token: MoaOrDcdtTokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> ContractCallWithMoaOrSingleDcdt<SA, OriginalResult> {
        self.with_moa_or_single_dcdt_transfer((payment_token, payment_nonce, payment_amount))
    }

    pub fn into_function_call(self) -> FunctionCall<SA> {
        self.function_call
    }

    pub fn tx(self) -> Tx<TxScEnv<SA>, (), (), (), (), FunctionCall<SA>, ()> {
        Tx::new_tx_from_sc().raw_data(self.function_call)
    }
}
