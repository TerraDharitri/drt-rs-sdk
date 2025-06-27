use crate::{
    imports::Bech32Address,
    scenario_model::{Account, BytesKey, BytesValue, Scenario, SetStateStep},
};

use dharitri_chain_scenario_format::interpret_trait::IntoRaw;
use dharitri_sdk::{
    blockchain::CommunicationProxy,
    data::{address::Address, dcdt::DcdtBalance},
};
use std::collections::{BTreeMap, HashMap};

pub async fn print_account_as_scenario_set_state(
    api: &CommunicationProxy,
    address: &Bech32Address,
) {
    let set_state = retrieve_account_as_scenario_set_state(api, address).await;
    let scenario = build_scenario(set_state);
    println!("{}", scenario.into_raw().to_json_string());
}

fn build_scenario(set_state: SetStateStep) -> Scenario {
    Scenario {
        name: None,
        comment: None,
        check_gas: None,
        steps: vec![crate::scenario_model::Step::SetState(set_state)],
    }
}

pub async fn retrieve_account_as_scenario_set_state(
    api: &CommunicationProxy,
    address: &Bech32Address,
) -> SetStateStep {
    let sdk_address = Address::from_bech32_string(address.to_bech32_str()).unwrap();
    let sdk_account = api.get_account(&sdk_address).await.unwrap();

    let account_dcdt = api
        .get_account_dcdt_tokens(&sdk_address)
        .await
        .unwrap_or_else(|err| {
            panic!("failed to retrieve DCDT tokens for address {address}: {err}")
        });
    let account_dcdt_roles = api
        .get_account_dcdt_roles(&sdk_address)
        .await
        .unwrap_or_else(|err| panic!("failed to retrieve DCDT roles for address {address}: {err}"));
    let account_storage = api
        .get_account_storage_keys(&sdk_address)
        .await
        .unwrap_or_else(|err| panic!("failed to retrieve storage for address {address}: {err}"));

    let account_state = set_account(
        sdk_account,
        account_storage,
        account_dcdt,
        account_dcdt_roles,
    );

    let set_state_step = SetStateStep::new();
    set_state_step.put_account(address, account_state)
}

pub fn set_account(
    account: dharitri_sdk::data::account::Account,
    account_storage: HashMap<String, String>,
    account_dcdt: HashMap<String, DcdtBalance>,
    account_dcdt_roles: HashMap<String, Vec<String>>,
) -> Account {
    let mut account_state = Account::new()
        .nonce(account.nonce)
        .balance(account.balance.as_str())
        .code(account.code);
    account_state.username = Some(account.username.as_str().into());
    account_state.storage = convert_storage(account_storage);

    for (_, dcdt_balance) in account_dcdt.iter() {
        let token_id_expr = format!("str:{}", dcdt_balance.token_identifier);
        account_state =
            account_state.dcdt_balance(token_id_expr.as_str(), dcdt_balance.balance.as_str());
    }

    for (token_id, dcdt_roles) in account_dcdt_roles {
        let token_id_expr = format!("str:{token_id}");
        account_state = account_state.dcdt_roles(token_id_expr.as_str(), dcdt_roles);
    }

    account_state
}

fn convert_storage(account_storage: HashMap<String, String>) -> BTreeMap<BytesKey, BytesValue> {
    account_storage
        .into_iter()
        .filter(|(k, _)| !k.starts_with("4e554d424154"))
        .map(|(k, v)| (BytesKey::from(k.as_str()), BytesValue::from(v)))
        .collect()
}
