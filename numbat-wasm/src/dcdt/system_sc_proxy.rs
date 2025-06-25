use core::marker::PhantomData;

use super::properties::*;
use hex_literal::hex;

use crate::{
    api::{CallTypeApi, SendApi},
    types::{
        BigUint, ContractCall, ContractCallNoPayment, ContractCallWithRewa, DcdtLocalRole,
        DcdtTokenType, ManagedAddress, ManagedBuffer, TokenIdentifier,
    },
};

/// Address of the system smart contract that manages DCDT.
/// Bech32: drt1yvesqqqqqqqqqqqqqqqqqqqqqqqqyvesqqqqqqqqqqqqqqqzlllsd5j0s2
pub const DCDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] =
    hex!("233300000000000000000000000000000002333000000000000000000002ffff");

const ISSUE_FUNGIBLE_ENDPOINT_NAME: &str = "issue";
const ISSUE_NON_FUNGIBLE_ENDPOINT_NAME: &str = "issueNonFungible";
const ISSUE_SEMI_FUNGIBLE_ENDPOINT_NAME: &str = "issueSemiFungible";
const REGISTER_META_DCDT_ENDPOINT_NAME: &str = "registerMetaDCDT";
const ISSUE_AND_SET_ALL_ROLES_ENDPOINT_NAME: &str = "registerAndSetAllRoles";

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
    ) -> ContractCallWithRewa<SA, ()> {
        self.issue(
            issue_cost,
            DcdtTokenType::Fungible,
            token_display_name,
            token_ticker,
            initial_supply,
            TokenProperties {
                num_decimals: properties.num_decimals,
                can_freeze: properties.can_freeze,
                can_wipe: properties.can_wipe,
                can_pause: properties.can_pause,
                can_transfer_create_role: false,
                can_mint: properties.can_mint,
                can_burn: properties.can_burn,
                can_change_owner: properties.can_change_owner,
                can_upgrade: properties.can_upgrade,
                can_add_special_roles: properties.can_add_special_roles,
            },
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
    ) -> ContractCallWithRewa<SA, ()> {
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
                can_transfer_create_role: properties.can_transfer_create_role,
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
    ) -> ContractCallWithRewa<SA, ()> {
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
                can_transfer_create_role: properties.can_transfer_create_role,
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
    ) -> ContractCallWithRewa<SA, ()> {
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
                can_transfer_create_role: properties.can_transfer_create_role,
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
    ) -> ContractCallWithRewa<SA, ()> {
        let dcdt_system_sc_address = self.dcdt_system_sc_address();

        let mut contract_call = ContractCallWithRewa::new(
            dcdt_system_sc_address,
            ISSUE_AND_SET_ALL_ROLES_ENDPOINT_NAME,
            issue_cost,
        );

        contract_call.proxy_arg(&token_display_name);
        contract_call.proxy_arg(&token_ticker);

        let token_type_name = match token_type {
            DcdtTokenType::Fungible => "FNG",
            DcdtTokenType::NonFungible => "NFT",
            DcdtTokenType::SemiFungible => "SFT",
            DcdtTokenType::Meta => "META",
            DcdtTokenType::Invalid => "",
        };
        contract_call.proxy_arg(&token_type_name);
        contract_call.proxy_arg(&num_decimals);

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
    ) -> ContractCallWithRewa<SA, ()> {
        let dcdt_system_sc_address = self.dcdt_system_sc_address();

        let endpoint_name = match token_type {
            DcdtTokenType::Fungible => ISSUE_FUNGIBLE_ENDPOINT_NAME,
            DcdtTokenType::NonFungible => ISSUE_NON_FUNGIBLE_ENDPOINT_NAME,
            DcdtTokenType::SemiFungible => ISSUE_SEMI_FUNGIBLE_ENDPOINT_NAME,
            DcdtTokenType::Meta => REGISTER_META_DCDT_ENDPOINT_NAME,
            DcdtTokenType::Invalid => "",
        };

        let mut contract_call =
            ContractCallWithRewa::new(dcdt_system_sc_address, endpoint_name, issue_cost);

        contract_call.proxy_arg(token_display_name);
        contract_call.proxy_arg(token_ticker);

        if token_type == DcdtTokenType::Fungible {
            contract_call.proxy_arg(initial_supply);
            contract_call.proxy_arg(&properties.num_decimals);
        } else if token_type == DcdtTokenType::Meta {
            contract_call.proxy_arg(&properties.num_decimals);
        }

        let mut token_prop_args = TokenPropertyArguments {
            can_freeze: Some(properties.can_freeze),
            can_wipe: Some(properties.can_wipe),
            can_pause: Some(properties.can_pause),
            can_change_owner: Some(properties.can_change_owner),
            can_upgrade: Some(properties.can_upgrade),
            can_add_special_roles: Some(properties.can_add_special_roles),
            ..TokenPropertyArguments::default()
        };

        if token_type == DcdtTokenType::Fungible {
            token_prop_args.can_mint = Some(properties.can_mint);
            token_prop_args.can_burn = Some(properties.can_burn);
        } else {
            token_prop_args.can_transfer_create_role = Some(properties.can_transfer_create_role);
        }

        append_token_property_arguments(&mut contract_call, &token_prop_args);

        contract_call
    }

    /// Produces a contract call to the DCDT system SC,
    /// which causes it to mint more fungible DCDT tokens.
    /// It will fail if the SC is not the owner of the token.
    pub fn mint(
        self,
        token_identifier: &TokenIdentifier<SA>,
        amount: &BigUint<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("mint");

        contract_call.proxy_arg(token_identifier);
        contract_call.proxy_arg(amount);

        contract_call
    }

    /// Produces a contract call to the DCDT system SC,
    /// which causes it to burn fungible DCDT tokens owned by the SC.
    pub fn burn(
        self,
        token_identifier: &TokenIdentifier<SA>,
        amount: &BigUint<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("DCDTBurn");

        contract_call.proxy_arg(token_identifier);
        contract_call.proxy_arg(amount);

        contract_call
    }

    /// The manager of an DCDT token may choose to suspend all transactions of the token,
    /// except minting, freezing/unfreezing and wiping.
    pub fn pause(self, token_identifier: &TokenIdentifier<SA>) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("pause");

        contract_call.proxy_arg(token_identifier);

        contract_call
    }

    /// The reverse operation of `pause`.
    pub fn unpause(self, token_identifier: &TokenIdentifier<SA>) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("unPause");

        contract_call.proxy_arg(token_identifier);

        contract_call
    }

    /// The manager of an DCDT token may freeze the tokens held by a specific account.
    /// As a consequence, no tokens may be transferred to or from the frozen account.
    /// Freezing and unfreezing the tokens of an account are operations designed to help token managers to comply with regulations.
    pub fn freeze(
        self,
        token_identifier: &TokenIdentifier<SA>,
        address: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("freeze");

        contract_call.proxy_arg(token_identifier);
        contract_call.proxy_arg(address);

        contract_call
    }

    /// The reverse operation of `freeze`, unfreezing, will allow further transfers to and from the account.
    pub fn unfreeze(
        self,
        token_identifier: &TokenIdentifier<SA>,
        address: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("unFreeze");

        contract_call.proxy_arg(token_identifier);
        contract_call.proxy_arg(address);

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
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("wipe");

        contract_call.proxy_arg(token_identifier);
        contract_call.proxy_arg(address);

        contract_call
    }

    /// This function converts an SFT to a metaDCDT by adding decimals to its structure in the metachain DCDT System SC.
    /// This function as almost all in case of DCDT can be called only by the owner.
    pub fn change_sft_to_meta_dcdt(
        self,
        token_identifier: &TokenIdentifier<SA>,
        num_decimals: usize,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("changeSFTToMetaDCDT");

        contract_call.proxy_arg(&token_identifier);
        contract_call.proxy_arg(&num_decimals);

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
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("setSpecialRole");

        contract_call.proxy_arg(token_identifier);
        contract_call.proxy_arg(address);
        for role in roles_iter {
            if role != DcdtLocalRole::None {
                contract_call.push_raw_argument(role.as_role_name());
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
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("unSetSpecialRole");

        contract_call.proxy_arg(token_identifier);
        contract_call.proxy_arg(address);
        for role in roles_iter {
            if role != DcdtLocalRole::None {
                contract_call.push_raw_argument(role.as_role_name());
            }
        }

        contract_call
    }

    pub fn transfer_ownership(
        self,
        token_identifier: &TokenIdentifier<SA>,
        new_owner: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("transferOwnership");

        contract_call.proxy_arg(token_identifier);
        contract_call.proxy_arg(new_owner);

        contract_call
    }

    pub fn transfer_nft_create_role(
        self,
        token_identifier: &TokenIdentifier<SA>,
        old_creator: &ManagedAddress<SA>,
        new_creator: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("transferNFTCreateRole");

        contract_call.proxy_arg(token_identifier);
        contract_call.proxy_arg(old_creator);
        contract_call.proxy_arg(new_creator);

        contract_call
    }

    pub fn control_changes(
        self,
        token_identifier: &TokenIdentifier<SA>,
        property_arguments: &TokenPropertyArguments,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self.dcdt_system_sc_call_no_args("controlChanges");
        contract_call.proxy_arg(token_identifier);
        append_token_property_arguments(&mut contract_call, property_arguments);
        contract_call
    }

    pub fn dcdt_system_sc_address(&self) -> ManagedAddress<SA> {
        ManagedAddress::new_from_bytes(&DCDT_SYSTEM_SC_ADDRESS_ARRAY)
    }

    fn dcdt_system_sc_call_no_args(
        self,
        endpoint_name: &'static str,
    ) -> ContractCallNoPayment<SA, ()> {
        let dcdt_system_sc_address = self.dcdt_system_sc_address();
        ContractCallNoPayment::new(dcdt_system_sc_address, endpoint_name)
    }
}

const TRUE_STR: &str = "true";
const FALSE_STR: &str = "false";

fn bool_name_bytes(b: bool) -> &'static str {
    if b {
        TRUE_STR
    } else {
        FALSE_STR
    }
}

fn set_token_property<SA, CC>(contract_call: &mut CC, name: &str, value: bool)
where
    SA: CallTypeApi + 'static,
    CC: ContractCall<SA>,
{
    contract_call.push_raw_argument(name);
    contract_call.push_raw_argument(bool_name_bytes(value));
}

fn append_token_property_arguments<SA, CC>(
    contract_call: &mut CC,
    token_prop_args: &TokenPropertyArguments,
) where
    SA: CallTypeApi + 'static,
    CC: ContractCall<SA>,
{
    if let Some(can_freeze) = token_prop_args.can_freeze {
        set_token_property(contract_call, "canFreeze", can_freeze);
    }

    if let Some(can_wipe) = token_prop_args.can_wipe {
        set_token_property(contract_call, "canWipe", can_wipe);
    }

    if let Some(can_pause) = token_prop_args.can_pause {
        set_token_property(contract_call, "canPause", can_pause);
    }

    if let Some(can_transfer_create_role) = token_prop_args.can_transfer_create_role {
        set_token_property(
            contract_call,
            "canTransferNFTCreateRole",
            can_transfer_create_role,
        );
    }

    if let Some(can_mint) = token_prop_args.can_mint {
        set_token_property(contract_call, "canMint", can_mint);
    }

    if let Some(can_burn) = token_prop_args.can_burn {
        set_token_property(contract_call, "canBurn", can_burn);
    }

    if let Some(can_change_owner) = token_prop_args.can_change_owner {
        set_token_property(contract_call, "canChangeOwner", can_change_owner);
    }

    if let Some(can_upgrade) = token_prop_args.can_upgrade {
        set_token_property(contract_call, "canUpgrade", can_upgrade);
    }

    if let Some(can_add_special_roles) = token_prop_args.can_add_special_roles {
        set_token_property(contract_call, "canAddSpecialRoles", can_add_special_roles);
    }
}
