// Code generated by the dharitri-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use dharitri_sc::proxy_imports::*;

pub struct ScenarioTesterProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for ScenarioTesterProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = ScenarioTesterProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        ScenarioTesterProxyMethods { wrapped_tx: tx }
    }
}

pub struct ScenarioTesterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> ScenarioTesterProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    /// Return value for testing reasons. 
    pub fn init<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        initial_value: Arg0,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, &'static str> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&initial_value)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> ScenarioTesterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        initial_value: Arg0,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .argument(&initial_value)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> ScenarioTesterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn sum(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getSum")
            .original_result()
    }

    pub fn other_mapper(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getOtherMapper")
            .original_result()
    }

    /// Add desired amount to the storage variable. 
    pub fn add<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        value: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("add")
            .argument(&value)
            .original_result()
    }

    /// Sets a value at another key 
    pub fn set_other_mapper<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        value: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("set_other_mapper")
            .argument(&value)
            .original_result()
    }

    /// Tests "from" conversion for MultiValueN parameters 
    pub fn multi_param<
        Arg0: ProxyArg<MultiValue2<BigUint<Env::Api>, BigUint<Env::Api>>>,
    >(
        self,
        _value: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("multi_param")
            .argument(&_value)
            .original_result()
    }

    /// Tests "from" conversion for MultiValueN return function 
    pub fn multi_return<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        value: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue2<BigUint<Env::Api>, BigUint<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("multi_return")
            .argument(&value)
            .original_result()
    }

    pub fn sc_panic(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("sc_panic")
            .original_result()
    }
}
