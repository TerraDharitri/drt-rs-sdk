use dharitri_chain_core::types::{BLSKey, BLSSignature};
use dharitri_sc_codec::multi_types::{MultiValue2, MultiValueVec};

use crate::types::{
    BigUint, Rewa, ProxyArg, Tx, TxRewaValue, TxEnv, TxFrom, TxGas, TxProxyTrait, TxTo, TxTypedCall,
};

/// Proxy for the Validator system smart contract.
pub struct ValidatorSCProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for ValidatorSCProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = ValidatorSCProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        ValidatorSCProxyMethods { wrapped_tx: tx }
    }
}

/// Method container of the Validator system smart contract proxy.
pub struct ValidatorSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

impl<Env, From, To, Gas> ValidatorSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// amount required for staking is 2500 REWA per BLS key
    pub fn stake<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<MultiValueVec<MultiValue2<BLSKey, BLSSignature>>>,
        RewaValue: TxRewaValue<Env>,
    >(
        self,
        max_nodes_to_run: Arg0,
        bls_keys_signatures: Arg1,
        amount: RewaValue,
    ) -> TxTypedCall<Env, From, To, Rewa<RewaValue>, Gas, ()> {
        self.wrapped_tx
            .raw_call("stake")
            .argument(&max_nodes_to_run)
            .argument(&bls_keys_signatures)
            .rewa(amount)
            .original_result()
    }
}
