use std::collections::BTreeMap;

use numbat_wasm::types::Address;
use denali::model::{Account, AddressKey, BlockInfo, NewAddress};
use num_bigint::BigUint;

use crate::world_mock::{
    is_smart_contract_address, AccountData, AccountDcdt, BlockInfo as CrateBlockInfo,
    BlockchainMock, DcdtData, DcdtInstance, DcdtInstanceMetadata, DcdtInstances, DcdtRoles,
};

pub fn execute(
    state: &mut BlockchainMock,
    accounts: &BTreeMap<AddressKey, Account>,
    new_addresses: &[NewAddress],
    previous_block_info: &Option<BlockInfo>,
    current_block_info: &Option<BlockInfo>,
) {
    for (address, account) in accounts.iter() {
        let storage = account
            .storage
            .iter()
            .map(|(k, v)| (k.value.clone(), v.value.clone()))
            .collect();
        let dcdt = AccountDcdt::new_from_raw_map(
            account
                .dcdt
                .iter()
                .map(|(k, v)| {
                    (
                        k.value.clone(),
                        convert_denali_dcdt_to_world_mock(k.value.as_slice(), v),
                    )
                })
                .collect(),
        );

        state.validate_and_add_account(AccountData {
            address: address.value.into(),
            nonce: account
                .nonce
                .as_ref()
                .map(|nonce| nonce.value)
                .unwrap_or_default(),
            rewa_balance: account
                .balance
                .as_ref()
                .map(|balance| balance.value.clone())
                .unwrap_or_default(),
            dcdt,
            username: account
                .username
                .as_ref()
                .map(|bytes_value| bytes_value.value.clone())
                .unwrap_or_default(),
            storage,
            contract_path: account
                .code
                .as_ref()
                .map(|bytes_value| bytes_value.value.clone()),
            contract_owner: account
                .owner
                .as_ref()
                .map(|address_value| address_value.value.into()),
        });
    }
    for new_address in new_addresses.iter() {
        assert!(
            is_smart_contract_address(&new_address.new_address.value.into()),
            "field should have SC format"
        );
        state.put_new_address(
            new_address.creator_address.value.into(),
            new_address.creator_nonce.value,
            new_address.new_address.value.into(),
        )
    }
    if let Some(block_info_obj) = &*previous_block_info {
        update_block_info(&mut state.previous_block_info, block_info_obj);
    }
    if let Some(block_info_obj) = &*current_block_info {
        update_block_info(&mut state.current_block_info, block_info_obj);
    }
}

fn convert_denali_dcdt_to_world_mock(
    token_identifier: &[u8],
    denali_dcdt: &denali::model::Dcdt,
) -> DcdtData {
    match denali_dcdt {
        denali::model::Dcdt::Short(short_dcdt) => {
            let balance = BigUint::from_bytes_be(short_dcdt.value.as_slice());
            let mut dcdt_data = DcdtData {
                token_identifier: token_identifier.to_vec(),
                ..Default::default()
            };
            dcdt_data.instances.add(0, balance);
            dcdt_data
        },
        denali::model::Dcdt::Full(full_dcdt) => DcdtData {
            token_identifier: full_dcdt
                .token_identifier
                .as_ref()
                .map(|token_identifier| token_identifier.value.clone())
                .unwrap_or_default(),
            instances: DcdtInstances::new_from_hash(
                full_dcdt
                    .instances
                    .iter()
                    .map(|denali_instance| {
                        let mock_instance =
                            convert_denali_dcdt_instance_to_world_mock(denali_instance);
                        (mock_instance.nonce, mock_instance)
                    })
                    .collect(),
            ),
            last_nonce: full_dcdt
                .last_nonce
                .as_ref()
                .map(|last_nonce| last_nonce.value)
                .unwrap_or_default(),
            roles: DcdtRoles::new(
                full_dcdt
                    .roles
                    .iter()
                    .map(|role| role.value.clone())
                    .collect(),
            ),
            frozen: if let Some(u64_value) = &full_dcdt.frozen {
                u64_value.value > 0
            } else {
                false
            },
        },
    }
}

fn convert_denali_dcdt_instance_to_world_mock(
    denali_dcdt: &denali::model::DcdtInstance,
) -> DcdtInstance {
    DcdtInstance {
        nonce: denali_dcdt
            .nonce
            .as_ref()
            .map(|nonce| nonce.value)
            .unwrap_or_default(),
        balance: denali_dcdt
            .balance
            .as_ref()
            .map(|value| value.value.clone())
            .unwrap_or_default(),
        metadata: DcdtInstanceMetadata {
            name: Vec::new(),
            creator: denali_dcdt
                .creator
                .as_ref()
                .map(|creator| Address::from_slice(creator.value.as_slice())),
            royalties: denali_dcdt
                .royalties
                .as_ref()
                .map(|royalties| royalties.value)
                .unwrap_or_default(),
            hash: denali_dcdt.hash.as_ref().map(|hash| hash.value.clone()),
            uri: denali_dcdt
                .uri
                .iter()
                .map(|uri| uri.value.clone())
                .collect(),
            attributes: denali_dcdt
                .attributes
                .as_ref()
                .map(|attributes| attributes.value.clone())
                .unwrap_or_default(),
        },
    }
}

fn update_block_info(
    block_info: &mut CrateBlockInfo,
    denali_block_info: &denali::model::BlockInfo,
) {
    if let Some(u64_value) = &denali_block_info.block_timestamp {
        block_info.block_timestamp = u64_value.value;
    }
    if let Some(u64_value) = &denali_block_info.block_nonce {
        block_info.block_nonce = u64_value.value;
    }
    if let Some(u64_value) = &denali_block_info.block_epoch {
        block_info.block_epoch = u64_value.value;
    }
    if let Some(u64_value) = &denali_block_info.block_round {
        block_info.block_round = u64_value.value;
    }
    if let Some(bytes_value) = &denali_block_info.block_random_seed {
        const SEED_LEN: usize = 48;
        let val = &bytes_value.value;

        assert!(
            val.len() == SEED_LEN,
            "block random seed input value must be exactly 48 bytes long"
        );

        let mut seed = [0u8; SEED_LEN];
        seed[..].copy_from_slice(val.as_slice());
        block_info.block_random_seed = Box::from(seed);
    }
}
