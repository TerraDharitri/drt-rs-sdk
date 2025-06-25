use core::marker::PhantomData;

use super::properties::*;
use hex_literal::hex;

use crate::{
    api::{CallTypeApi, SendApi},
    types::{
        Address, BigUint, ContractCall, DcdtLocalRole, DcdtTokenType, ManagedAddress,
        ManagedBuffer, TokenIdentifier,
    },
};

/// Address of the system smart contract that manages DCDT.
/// Bech32: drt1yvesqqqqqqqqqqqqqqqqqqqqqqqqyvesqqqqqqqqqqqqqqqzlllsd5j0s2
pub const DCDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] =
    hex!("233300000000000000000000000000000002333000000000000000000002ffff");

const ISSUE_FUNGIBLE_ENDPOINT_NAME: &[u8] = b"issue";
const ISSUE_NON_FUNGIBLE_ENDPOINT_NAME: &[u8] = b"issueNonFungible";
const ISSUE_SEMI_FUNGIBLE_ENDPOINT_NAME: &[u8] = b"issueSemiFungible";
const REGISTER_META_DCDT_ENDPOINT_NAME: &[u8] = b"registerMetaDCDT";
const ISSUE_AND_SET_ALL_ROLES_ENDPOINT_NAME: &[u8] = b"registerAndSetAllRoles";

/// Proxy for the DCDT system smart contract.
/// Unlike other contract proxies, this one has a fixed address,
/// so the proxy object doesn't really contain any data, it is more of a placeholder.
pub struct DCDTSystemSmartContractProxy<SA>
where
    SA: SendApi + 'static,
{
    _phantom: PhantomData<SA>,
}

impl<SA> DCDTSystemSmartContractProxy<SA>
where
    SA: SendApi + 'static,
{
    /// Constructor.
    /// TODO: consider moving this to a new Proxy contructor trait (bonus: better proxy constructor syntax).
    pub fn new_proxy_obj() -> Self {
        DCDTSystemSmartContractProxy {
            _phantom: PhantomData,
        }
    }
}

impl<SA> DCDTSystemSmartContractProxy<SA>
where
    SA: CallTypeApi + 'static,
{
    /// Produces a contract call to the DCDT system SC,
    /// which causes it to issue a new fungible DCDT token.
    pub fn issue_fungible(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        initial_supply: &BigUint<SA>,
        properties: FungibleTokenProperties,
    ) -> ContractCall<SA, ()> {
        self.issue(
            issue_cost,
            DcdtTokenType::Fungible,
            token_display_name,
            token_ticker,
            initial_supply,
            properties,
        )
    }

    /// Produces a contract call to the DCDT system SC,
    /// which causes it to issue a new non-fungible DCDT token.
    pub fn issue_non_fungible(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        properties: NonFungibleTokenProperties,
    ) -> ContractCall<SA, ()> {
        let zero = BigUint::zero();
        self.issue(
            issue_cost,
            DcdtTokenType::NonFungible,
            token_display_name,
            token_ticker,
            &zero,
            TokenProperties {
                num_decimals: 0,
                can_freeze: properties.can_freeze,
                can_wipe: properties.can_wipe,
                can_pause: properties.can_pause,
                can_mint: false,
                can_burn: false,
                can_change_owner: properties.can_change_owner,
                can_upgrade: properties.can_upgrade,
                can_add_special_roles: properties.can_add_special_roles,
            },
        )
    }

    /// Produces a contract call to the DCDT system SC,
    /// which causes it to issue a new semi-fungible DCDT token.
    pub fn issue_semi_fungible(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        properties: SemiFungibleTokenProperties,
    ) -> ContractCall<SA, ()> {
        let zero = BigUint::zero();
        self.issue(
            issue_cost,
            DcdtTokenType::SemiFungible,
            token_display_name,
            token_ticker,
            &zero,
            TokenProperties {
                num_decimals: 0,
                can_freeze: properties.can_freeze,
                can_wipe: properties.can_wipe,
                can_pause: properties.can_pause,
                can_mint: false,
                can_burn: false,
                can_change_owner: properties.can_change_owner,
                can_upgrade: properties.can_upgrade,
                can_add_special_roles: properties.can_add_special_roles,
            },
        )
    }

    /// Produces a contract call to the DCDT system SC,
    /// which causes it to register a new Meta DCDT token.
    pub fn register_meta_dcdt(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        properties: MetaTokenProperties,
    ) -> ContractCall<SA, ()> {
        let zero = BigUint::zero();
        self.issue(
            issue_cost,
            DcdtTokenType::Meta,
            token_display_name,
            token_ticker,
            &zero,
            TokenProperties {
                num_decimals: properties.num_decimals,
                can_freeze: properties.can_freeze,
                can_wipe: properties.can_wipe,
                can_pause: properties.can_pause,
                can_mint: false,
                can_burn: false,
                can_change_owner: properties.can_change_owner,
                can_upgrade: properties.can_upgrade,
                can_add_special_roles: properties.can_add_special_roles,
            },
        )
    }

    pub fn issue_and_set_all_roles(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
        token_type: DcdtTokenType,
        num_decimals: usize,
    ) -> ContractCall<SA, ()> {
        let dcdt_system_sc_address = self.dcdt_system_sc_address();

        let mut contract_call = ContractCall::new(
            dcdt_system_sc_address,
            ManagedBuffer::new_from_bytes(ISSUE_AND_SET_ALL_ROLES_ENDPOINT_NAME),
        )
        .with_rewa_transfer(issue_cost);

        contract_call.push_endpoint_arg(&token_display_name);
        contract_call.push_endpoint_arg(&token_ticker);

        let token_type_name = match token_type {
            DcdtTokenType::Fungible => &b"FNG"[..],
            DcdtTokenType::NonFungible => &b"NFT"[..],
            DcdtTokenType::SemiFungible => &b"SFT"[..],
            DcdtTokenType::Meta => &b"META"[..],
            DcdtTokenType::Invalid => &[],
        };
        contract_call.push_endpoint_arg(&token_type_name);
        contract_call.push_endpoint_arg(&num_decimals);

        contract_call
    }

    /// Deduplicates code from all the possible issue functions
    fn issue(
        self,
        issue_cost: BigUint<SA>,
        token_type: DcdtTokenType,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        initial_supply: &BigUint<SA>,
        properties: TokenProperties,
    ) -> ContractCall<SA, ()> {
        let dcdt_system_sc_address = self.dcdt_system_sc_address();

        let endpoint_name = match token_type {
            DcdtTokenType::Fungible => ISSUE_FUNGIBLE_ENDPOINT_NAME,
            DcdtTokenType::NonFungible => ISSUE_NON_FUNGIBLE_ENDPOINT_NAME,
            DcdtTokenType::SemiFungible => ISSUE_SEMI_FUNGIBLE_ENDPOINT_NAME,
            DcdtTokenType::Meta => REGISTER_META_DCDT_ENDPOINT_NAME,
            DcdtTokenType::Invalid => &[],
        };

        let mut contract_call = ContractCall::new(
            dcdt_system_sc_address,
            ManagedBuffer::new_from_bytes(endpoint_name),
        )
        .with_rewa_transfer(issue_cost);

        contract_call.push_endpoint_arg(token_display_name);
        contract_call.push_endpoint_arg(token_ticker);

        if token_type == DcdtTokenType::Fungible {
            contract_call.push_endpoint_arg(initial_supply);
            contract_call.push_endpoint_arg(&properties.num_decimals);
        } else if token_type == DcdtTokenType::Meta {
            contract_call.push_endpoint_arg(&properties.num_decimals);
        }

        set_token_property(&mut contract_call, &b"canFreeze"[..], properties.can_freeze);
        set_token_property(&mut contract_call, &b"canWipe"[..], properties.can_wipe);
        set_token_property(&mut contract_call, &b"canPause"[..], properties.can_pause);

        if token_type == DcdtTokenType::Fungible {
            set_token_property(&mut contract_call, &b"canMint"[..], properties.can_mint);
            set_token_property(&mut contract_call, &b"canBurn"[..], properties.can_burn);
        }

        set_token_property(
            &mut contract_call,
            &b"canChangeOwner"[..],
            properties.can_change_owner,
        );
        set_token_property(
            &mut contract_call,
            &b"canUpgrade"[..],
            properties.can_upgrade,
        );
        set_token_property(
            &mut contract_call,
            &b"canAddSpecialRoles"[..],
            properties.can_add_special_roles,
        );

        contract_call
    }

    /// Produces a contract call to the DCDT system SC,
    /// which causes it to mint more fungible DCDT tokens.
    /// It will fail if the SC is not the owner of the token.
    pub fn mint(
        self,
        token_identifier: &TokenIdentifier<SA>,
        amount: &BigUint<SA>,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"mint");

        contract_call.push_endpoint_arg(token_identifier);
        contract_call.push_endpoint_arg(amount);

        contract_call
    }

    /// Produces a contract call to the DCDT system SC,
    /// which causes it to burn fungible DCDT tokens owned by the SC.
    pub fn burn(
        self,
        token_identifier: &TokenIdentifier<SA>,
        amount: &BigUint<SA>,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"DCDTBurn");

        contract_call.push_endpoint_arg(token_identifier);
        contract_call.push_endpoint_arg(amount);

        contract_call
    }

    /// The manager of an DCDT token may choose to suspend all transactions of the token,
    /// except minting, freezing/unfreezing and wiping.
    pub fn pause(self, token_identifier: &TokenIdentifier<SA>) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"pause");

        contract_call.push_endpoint_arg(token_identifier);

        contract_call
    }

    /// The reverse operation of `pause`.
    pub fn unpause(self, token_identifier: &TokenIdentifier<SA>) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"unPause");

        contract_call.push_endpoint_arg(token_identifier);

        contract_call
    }

    /// The manager of an DCDT token may freeze the tokens held by a specific account.
    /// As a consequence, no tokens may be transferred to or from the frozen account.
    /// Freezing and unfreezing the tokens of an account are operations designed to help token managers to comply with regulations.
    pub fn freeze(
        self,
        token_identifier: &TokenIdentifier<SA>,
        address: &ManagedAddress<SA>,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"freeze");

        contract_call.push_endpoint_arg(token_identifier);
        contract_call.push_endpoint_arg(address);

        contract_call
    }

    /// The reverse operation of `freeze`, unfreezing, will allow further transfers to and from the account.
    pub fn unfreeze(
        self,
        token_identifier: &TokenIdentifier<SA>,
        address: &ManagedAddress<SA>,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"unFreeze");

        contract_call.push_endpoint_arg(token_identifier);
        contract_call.push_endpoint_arg(address);

        contract_call
    }

    /// The manager of an DCDT token may wipe out all the tokens held by a frozen account.
    /// This operation is similar to burning the tokens, but the account must have been frozen beforehand,
    /// and it must be done by the token manager.
    /// Wiping the tokens of an account is an operation designed to help token managers to comply with regulations.
    pub fn wipe(
        self,
        token_identifier: &TokenIdentifier<SA>,
        address: &ManagedAddress<SA>,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"wipe");

        contract_call.push_endpoint_arg(token_identifier);
        contract_call.push_endpoint_arg(address);

        contract_call
    }

    /// This function converts an SFT to a metaDCDT by adding decimals to its structure in the metachain DCDT System SC.
    /// This function as almost all in case of DCDT can be called only by the owner.
    pub fn change_sft_to_meta_dcdt(
        self,
        token_identifier: &TokenIdentifier<SA>,
        num_decimals: usize,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"changeSFTToMetaDCDT");

        contract_call.push_endpoint_arg(&token_identifier);
        contract_call.push_endpoint_arg(&num_decimals);

        contract_call
    }

    /// This function can be called only if canSetSpecialRoles was set to true.
    /// The metachain system SC will evaluate the arguments and call “DCDTSetRole@tokenId@listOfRoles” for the given address.
    /// This will be actually a cross shard call.
    /// This function as almost all in case of DCDT can be called only by the owner.
    pub fn set_special_roles<RoleIter: Iterator<Item = DcdtLocalRole>>(
        self,
        address: &ManagedAddress<SA>,
        token_identifier: &TokenIdentifier<SA>,
        roles_iter: RoleIter,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"setSpecialRole");

        contract_call.push_endpoint_arg(token_identifier);
        contract_call.push_endpoint_arg(address);
        for role in roles_iter {
            if role != DcdtLocalRole::None {
                contract_call.push_argument_raw_bytes(role.as_role_name());
            }
        }

        contract_call
    }

    /// This function can be called only if canSetSpecialRoles was set to true.
    /// The metachain system SC will evaluate the arguments and call “DCDTUnsetRole@tokenId@listOfRoles” for the given address.
    /// This will be actually a cross shard call.
    /// This function as almost all in case of DCDT can be called only by the owner.
    pub fn unset_special_roles<RoleIter: Iterator<Item = DcdtLocalRole>>(
        self,
        address: &ManagedAddress<SA>,
        token_identifier: &TokenIdentifier<SA>,
        roles_iter: RoleIter,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"unSetSpecialRole");

        contract_call.push_endpoint_arg(token_identifier);
        contract_call.push_endpoint_arg(address);
        for role in roles_iter {
            if role != DcdtLocalRole::None {
                contract_call.push_argument_raw_bytes(role.as_role_name());
            }
        }

        contract_call
    }

    pub fn transfer_ownership(
        self,
        token_identifier: &TokenIdentifier<SA>,
        new_owner: &Address,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"transferOwnership");

        contract_call.push_endpoint_arg(token_identifier);
        contract_call.push_argument_raw_bytes(new_owner.as_bytes());

        contract_call
    }

    pub fn transfer_nft_create_role(
        self,
        token_identifier: &TokenIdentifier<SA>,
        old_creator: &Address,
        new_creator: &Address,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args(b"transferNFTCreateRole");

        contract_call.push_endpoint_arg(token_identifier);
        contract_call.push_argument_raw_bytes(old_creator.as_bytes());
        contract_call.push_argument_raw_bytes(new_creator.as_bytes());

        contract_call
    }

    pub fn dcdt_system_sc_address(&self) -> ManagedAddress<SA> {
        ManagedAddress::new_from_bytes(&DCDT_SYSTEM_SC_ADDRESS_ARRAY)
    }

    fn dcdt_system_sc_call_no_args(self, endpoint_name: &[u8]) -> ContractCall<SA, ()> {
        let dcdt_system_sc_address = self.dcdt_system_sc_address();
        ContractCall::new(
            dcdt_system_sc_address,
            ManagedBuffer::new_from_bytes(endpoint_name),
        )
    }
}

const TRUE_BYTES: &[u8] = b"true";
const FALSE_BYTES: &[u8] = b"false";

fn bool_name_bytes(b: bool) -> &'static [u8] {
    if b {
        TRUE_BYTES
    } else {
        FALSE_BYTES
    }
}

fn set_token_property<SA, R>(contract_call: &mut ContractCall<SA, R>, name: &[u8], value: bool)
where
    SA: CallTypeApi + 'static,
{
    contract_call.push_argument_raw_bytes(name);
    contract_call.push_argument_raw_bytes(bool_name_bytes(value));
}
