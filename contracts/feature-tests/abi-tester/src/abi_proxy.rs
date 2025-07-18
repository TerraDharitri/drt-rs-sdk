// Code generated by the dharitri-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use dharitri_sc::proxy_imports::*;

pub struct AbiTesterProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for AbiTesterProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = AbiTesterProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        AbiTesterProxyMethods { wrapped_tx: tx }
    }
}

pub struct AbiTesterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> AbiTesterProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    /// Contract constructor. 
    pub fn init<
        Arg0: ProxyArg<i32>,
        Arg1: ProxyArg<OnlyShowsUpInConstructor>,
    >(
        self,
        _constructor_arg_1: Arg0,
        _constructor_arg_2: Arg1,
    ) -> TxTypedDeploy<Env, From, (), Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&_constructor_arg_1)
            .argument(&_constructor_arg_2)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> AbiTesterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Upgrade constructor. 
    pub fn upgrade<
        Arg0: ProxyArg<i32>,
        Arg1: ProxyArg<OnlyShowsUpInConstructor>,
    >(
        self,
        _constructor_arg_1: Arg0,
        _constructor_arg_2: Arg1,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .argument(&_constructor_arg_1)
            .argument(&_constructor_arg_2)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> AbiTesterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Example endpoint docs. 
    pub fn echo_abi_test_type<
        Arg0: ProxyArg<AbiTestType>,
    >(
        self,
        att: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, AbiTestType> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("echo_abi_test_type")
            .argument(&att)
            .original_result()
    }

    pub fn echo_enum<
        Arg0: ProxyArg<AbiEnum>,
    >(
        self,
        e: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, AbiEnum> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("echo_enum")
            .argument(&e)
            .original_result()
    }

    pub fn take_managed_type<
        Arg0: ProxyArg<AbiManagedType<Env::Api>>,
    >(
        self,
        _arg: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("take_managed_type")
            .argument(&_arg)
            .original_result()
    }

    pub fn multi_result_3(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue3<i32, [u8; 3], BoxedBytes>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("multi_result_3")
            .original_result()
    }

    pub fn multi_result_4(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue4<i32, [u8; 3], BoxedBytes, OnlyShowsUpAsNested03>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("multi_result_4")
            .original_result()
    }

    pub fn var_args<
        Arg0: ProxyArg<u32>,
        Arg1: ProxyArg<MultiValueVec<MultiValue2<OnlyShowsUpAsNested04, i32>>>,
    >(
        self,
        _simple_arg: Arg0,
        _var_args: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("var_args")
            .argument(&_simple_arg)
            .argument(&_var_args)
            .original_result()
    }

    pub fn multi_result_vec(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueVec<MultiValue3<OnlyShowsUpAsNested05, bool, ()>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("multi_result_vec")
            .original_result()
    }

    pub fn optional_arg<
        Arg0: ProxyArg<u32>,
        Arg1: ProxyArg<OptionalValue<OnlyShowsUpAsNested06>>,
    >(
        self,
        _simple_arg: Arg0,
        _opt_args: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("optional_arg")
            .argument(&_simple_arg)
            .argument(&_opt_args)
            .original_result()
    }

    pub fn optional_result(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<OnlyShowsUpAsNested07>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("optional_result")
            .original_result()
    }

    pub fn address_vs_h256<
        Arg0: ProxyArg<Address>,
        Arg1: ProxyArg<H256>,
    >(
        self,
        address: Arg0,
        h256: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue2<Address, H256>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("address_vs_h256")
            .argument(&address)
            .argument(&h256)
            .original_result()
    }

    pub fn managed_address_vs_byte_array<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedByteArray<Env::Api, 32usize>>,
    >(
        self,
        address: Arg0,
        byte_array: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue2<ManagedAddress<Env::Api>, ManagedByteArray<Env::Api, 32usize>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("managed_address_vs_byte_array")
            .argument(&address)
            .argument(&byte_array)
            .original_result()
    }

    pub fn process_managed_decimal<
        Arg0: ProxyArg<ManagedDecimal<Env::Api, ConstDecimals<U10>>>,
    >(
        self,
        input: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedDecimal<Env::Api, usize>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("process_managed_decimal")
            .argument(&input)
            .original_result()
    }

    pub fn dcdt_local_role(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, DcdtLocalRole> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("dcdt_local_role")
            .original_result()
    }

    pub fn dcdt_token_payment(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, DcdtTokenPayment<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("dcdt_token_payment")
            .original_result()
    }

    pub fn dcdt_token_data(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, DcdtTokenData<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("dcdt_token_data")
            .original_result()
    }

    pub fn sample_storage_mapper(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OnlyShowsUpAsNestedInSingleValueMapper> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("sample_storage_mapper")
            .original_result()
    }

    pub fn item_for_vec(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Vec<OnlyShowsUpAsNestedInVec>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("item_for_vec")
            .original_result()
    }

    pub fn item_for_array_vec(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ArrayVec<OnlyShowsUpAsNestedInArrayVec, 3usize>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("item_for_array_vec")
            .original_result()
    }

    pub fn item_for_managed_vec(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, AbiManagedVecItem>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("item_for_managed_vec")
            .original_result()
    }

    pub fn echo_permission<
        Arg0: ProxyArg<Permission>,
    >(
        self,
        p: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Permission> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("echo_permission")
            .argument(&p)
            .original_result()
    }

    pub fn item_for_array<
        Arg0: ProxyArg<[OnlyShowsUpAsNestedInArray; 5]>,
    >(
        self,
        _array: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("item_for_array")
            .argument(&_array)
            .original_result()
    }

    pub fn item_for_box(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Box<OnlyShowsUpAsNestedInBox>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("item_for_box")
            .original_result()
    }

    pub fn item_for_boxed_slice(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Box<[OnlyShowsUpAsNestedInBoxedSlice]>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("item_for_boxed_slice")
            .original_result()
    }

    pub fn item_for_ref<
        Arg0: ProxyArg<OnlyShowsUpAsNestedInRef>,
    >(
        self,
        _ref: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("item_for_ref")
            .argument(&_ref)
            .original_result()
    }

    pub fn item_for_slice<
        Arg0: ProxyArg<Box<[OnlyShowsUpAsNestedInSlice]>>,
    >(
        self,
        _ref: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("item_for_slice")
            .argument(&_ref)
            .original_result()
    }

    pub fn item_for_option(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Option<OnlyShowsUpAsNestedInOption>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("item_for_option")
            .original_result()
    }

    pub fn operation_completion_status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OperationCompletionStatus> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("operation_completion_status")
            .original_result()
    }

    pub fn takes_object_with_managed_buffer_read_to_end<
        Arg0: ProxyArg<AbiWithManagedBufferReadToEnd<Env::Api>>,
    >(
        self,
        arg: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("takes_object_with_managed_buffer_read_to_end")
            .argument(&arg)
            .original_result()
    }

    pub fn payable_rewa(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("payable_rewa")
            .original_result()
    }

    pub fn payable_some_token(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("payable_some_token")
            .original_result()
    }

    pub fn payable_any_token(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("payable_any_token")
            .original_result()
    }
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpInConstructor {
    pub something: (),
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct AbiTestType {
    pub nested: OnlyShowsUpAsNested01,
    pub next: Option<Box<AbiTestType>>,
    pub tuple_madness: (OnlyShowsUpAsNested02, Option<Box<AbiTestType>>),
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested01 {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested02 {
    pub something: [u8; 0],
}

#[rustfmt::skip]
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum AbiEnum {
    Nothing,
    Something(i32),
    SomethingMore(u8, OnlyShowsUpAsNested08),
    SomeStruct {
        a: u16,
        b: OnlyShowsUpAsNested09,
    },
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested08 {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested09 {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct AbiManagedType<Api>
where
    Api: ManagedTypeApi,
{
    pub big_uint: BigUint<Api>,
    pub integer: i32,
    pub managed_buffer: ManagedBuffer<Api>,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested03 {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested04 {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested05 {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested06 {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested07 {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInSingleValueMapper {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInVec {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInArrayVec {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, ManagedVecItem)]
pub struct AbiManagedVecItem {
    pub value1: u32,
    pub value2: u32,
}

#[type_abi]
#[derive(Clone, Copy, Debug, PartialEq, Eq, NestedDecode, NestedEncode, TopEncode, TopDecode)]
pub struct Permission(u32);

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInArray {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInBox {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInBoxedSlice {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInRef {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInSlice {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInOption {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct AbiWithManagedBufferReadToEnd<Api>
where
    Api: ManagedTypeApi,
{
    pub endpoint: ManagedBuffer<Api>,
    pub gas: u64,
    pub flush: ManagedBufferReadToEnd<Api>,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct OnlyShowsUpInDcdtAttr {
    pub field: OnlyShowsUpAsNested10,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested10 {}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum ExplicitDiscriminant {
    Zero,
    Thirty,
    Twelve,
    Fifty,
    FiftyOne,
}

#[rustfmt::skip]
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum ExplicitDiscriminantMixed {
    Zero,
    Unit,
    Tuple(u16),
    Five,
    Struct {
        a: u8,
        b: u16,
    },
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct ManagedDecimalWrapper<Api>
where
    Api: ManagedTypeApi,
{
    pub field: ManagedDecimal<Api, ConstDecimals<U2>>,
}
