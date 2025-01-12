use crate::{
    contract_base::SendRawWrapper,
    types::{
        AnnotatedValue, BigUint, ManagedAddress, ManagedBuffer, ManagedVec, TxFrom, TxToSpecified,
    },
};

use super::{
    AnnotatedMoaPayment, FullPaymentData, FunctionCall, TxMoaValue, TxEnv, TxPayment,
    TxPaymentMoaOnly,
};

/// Indicates the MOA payment in a transaction.
pub struct Moa<MoaValue>(pub MoaValue);

pub type MoaPayment<Api> = Moa<BigUint<Api>>;

impl<MoaValue: Clone> Clone for Moa<MoaValue> {
    fn clone(&self) -> Self {
        Moa(self.0.clone())
    }
}

impl<Env, MoaValue> TxPayment<Env> for Moa<MoaValue>
where
    Env: TxEnv,
    MoaValue: TxMoaValue<Env>,
{
    fn is_no_payment(&self, env: &Env) -> bool {
        self.0.with_value_ref(env, |moa_value| moa_value == &0u32)
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.0.with_value_ref(env, |moa_value| {
            let _ = SendRawWrapper::<Env::Api>::new().direct_moa_execute(
                to,
                moa_value,
                gas_limit,
                &fc.function_name,
                &fc.arg_buffer,
            );
        })
    }

    #[inline]
    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        _from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            self.0
                .with_value_ref(env, |moa_value| f(to_addr, moa_value, fc))
        })
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            moa: Some(AnnotatedMoaPayment::new_moa(self.0.into_value(env))),
            multi_dcdt: ManagedVec::new(),
        }
    }
}

impl<Env, MoaValue> AnnotatedValue<Env, BigUint<Env::Api>> for Moa<MoaValue>
where
    Env: TxEnv,
    MoaValue: TxMoaValue<Env>,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        self.0.annotation(env)
    }

    #[inline]
    fn to_value(&self, env: &Env) -> BigUint<Env::Api> {
        self.0.to_value(env)
    }

    #[inline]
    fn into_value(self, env: &Env) -> BigUint<Env::Api> {
        self.0.into_value(env)
    }

    #[inline]
    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        self.0.with_value_ref(env, f)
    }
}

impl<Env, MoaValue> TxPaymentMoaOnly<Env> for Moa<MoaValue>
where
    Env: TxEnv,
    MoaValue: TxMoaValue<Env>,
{
}