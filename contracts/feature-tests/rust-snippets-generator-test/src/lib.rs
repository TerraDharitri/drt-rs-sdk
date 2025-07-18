#![no_std]

dharitri_sc::imports!();
dharitri_sc::derive_imports!();

// contract to help with snippet generation tests
// contains endpoints with various types combinations
// TODO: Split into modules when testing the auto-imports

// To generate snippets, change dir to the `meta` folder and run:
// cargo run snippets
// Add --overwrite if you want to overwrite existing snippets

// Additionally, we also have to update the interactor snippets manually to add relative paths:
// [dependencies.dharitri-sc-snippets]
// path = "../../../../framework/snippets"

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Debug, Clone)]
pub struct MyCoolStruct<M: ManagedTypeApi> {
    pub awesome: BigUint<M>,
}

#[dharitri_sc::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn no_arg_no_result_endpoint(&self) {}

    #[endpoint]
    fn no_arg_one_result_endpoint(&self) -> u64 {
        0
    }

    #[endpoint]
    fn one_arg_no_result_endpoint(&self, _arg: u64) {}

    #[endpoint]
    fn one_arg_one_result_endpoint(&self, _arg: u64) -> BigUint {
        BigUint::zero()
    }

    #[endpoint]
    fn multi_result(&self, _arg: TokenIdentifier) -> MultiValueEncoded<BigUint> {
        MultiValueEncoded::new()
    }

    #[endpoint]
    fn nested_result(&self, _arg: TokenIdentifier) -> ManagedVec<ManagedVec<BigUint>> {
        ManagedVec::new()
    }

    #[endpoint]
    fn custom_struct(&self, _arg: MyCoolStruct<Self::Api>) -> MyCoolStruct<Self::Api> {
        MyCoolStruct {
            awesome: BigUint::zero(),
        }
    }

    #[endpoint]
    fn optional_type(&self, _arg: OptionalValue<BigUint>) -> OptionalValue<TokenIdentifier> {
        OptionalValue::None
    }

    #[endpoint]
    fn option_type(&self, _arg: Option<ManagedVec<TokenIdentifier>>) -> Option<u64> {
        None
    }

    #[endpoint]
    fn dcdt_token_payment(&self, _arg: OptionalValue<DcdtTokenPayment>) -> DcdtTokenPayment {
        DcdtTokenPayment::new(
            TokenIdentifier::from_dcdt_bytes(b"COOL-123456"),
            0,
            BigUint::from(1_000u64),
        )
    }

    #[endpoint]
    fn rewa_or_dcdt_payment(&self, arg: RewaOrDcdtTokenPayment) -> RewaOrDcdtTokenIdentifier {
        arg.token_identifier
    }

    #[payable("*")]
    #[endpoint]
    fn payable_endpoint(&self) {}

    #[endpoint]
    fn managed_buffer(
        &self,
        _arg: Option<ManagedBuffer>,
    ) -> MultiValueEncoded<ManagedVec<MyCoolStruct<Self::Api>>> {
        MultiValueEncoded::new()
    }

    #[endpoint]
    fn multi_value_2(&self, arg: MultiValue2<u64, BigUint>) -> MultiValue2<u64, BigUint> {
        arg
    }

    #[endpoint]
    fn multi_value_4(
        &self,
        arg: MultiValue4<u64, BigUint, MyCoolStruct<Self::Api>, TokenIdentifier>,
    ) -> MultiValue4<u64, BigUint, MyCoolStruct<Self::Api>, TokenIdentifier> {
        arg
    }

    #[endpoint]
    fn complex_multi_values(
        &self,
        arg: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) -> MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>> {
        arg
    }
}
