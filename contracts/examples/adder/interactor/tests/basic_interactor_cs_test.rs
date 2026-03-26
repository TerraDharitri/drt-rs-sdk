use basic_interactor::{AdderInteract, Config};
use dharitri_sc_snippets::{imports::Bech32Address, sdk::gateway::SetStateAccount, test_wallets};
use serial_test::serial;
use std::fs;
use std::io::Write;
use std::collections::HashMap;

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn simulator_upgrade_test() {
    let mut basic_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    basic_interact
        .interactor
        .generate_blocks(2u64)
        .await
        .unwrap();

    basic_interact.deploy().await;
    basic_interact.add(1u32).await;

    // Sum will be 1
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 1u32.into());

    basic_interact
        .upgrade(7u32, &basic_interact.adder_owner_address.clone(), None)
        .await;

    // Sum will be the updated value of 7
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 7u32.into());

    // Upgrade fails
    basic_interact
        .upgrade(
            10u32,
            &basic_interact.wallet_address.clone(),
            Some("upgrade is allowed only for owner"),
        )
        .await;

    // Sum will remain 7
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 7u32.into());
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn set_state_cs_test() {
    let account_address = test_wallets::mike();
    let simulator_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    // Create a mock account with some storage instead of fetching from real chain
    let mut set_state_account = SetStateAccount::from_address(
        Bech32Address::from(&account_address.to_address()).to_bech32_string(),
    );
    set_state_account.balance = "1000000000000000000".to_string(); // 1 REWA
    
    let mut storage = std::collections::HashMap::new();
    storage.insert("test_key".to_string(), "test_value".to_string());
    let set_state_account = set_state_account.with_storage(storage);
    
    let vec_state = vec![set_state_account];

    let set_state_response = simulator_interact.interactor.set_state(vec_state).await;

    simulator_interact
        .interactor
        .generate_blocks(2u64)
        .await
        .unwrap();

    assert!(set_state_response.is_ok());

    let storage = simulator_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage.len() >= 1);

    println!("mike's storage keys in chain simulator {:#?}", storage);
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn set_state_from_file_cs_test() {
    let account_address = test_wallets::mike();
    let account_address_2 = test_wallets::ivan();
    let simulator_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    // Create mock accounts and save them to file
    let mut account_1 = SetStateAccount::from_address(
        Bech32Address::from(&account_address.to_address()).to_bech32_string(),
    );
    account_1.balance = "1000000000000000000".to_string(); // 1 REWA
    let mut storage_1 = std::collections::HashMap::new();
    storage_1.insert("key1".to_string(), "value1".to_string());
    let account_1 = account_1.with_storage(storage_1);
    
    let mut account_2 = SetStateAccount::from_address(
        Bech32Address::from(&account_address_2.to_address()).to_bech32_string(),
    );
    account_2.balance = "2000000000000000000".to_string(); // 2 REWA
    let mut storage_2 = std::collections::HashMap::new();
    storage_2.insert("key2".to_string(), "value2".to_string());
    let account_2 = account_2.with_storage(storage_2);

    // Save accounts to file
    let accounts = vec![account_1, account_2];
    let file_path = simulator_interact.interactor.get_state_file_path();
    let file = std::fs::File::create(&file_path).expect("Failed to create state file");
    serde_json::to_writer_pretty(file, &accounts).expect("Failed to write accounts to file");

    let set_state_response = simulator_interact
        .interactor
        .set_state_for_saved_accounts()
        .await;

    simulator_interact
        .interactor
        .generate_blocks(2u64)
        .await
        .unwrap();

    assert!(set_state_response.is_ok());

    let storage = simulator_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage.len() >= 1);

    println!("mike's storage keys in chain simulator {:#?}", storage);
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn set_state_overwrite_cs_test() {
    let account_address = test_wallets::mike();
    let account_address_2 = test_wallets::ivan();
    let simulator_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    // Create mock accounts and save them to file
    let mut account_1 = SetStateAccount::from_address(
        Bech32Address::from(&account_address.to_address()).to_bech32_string(),
    );
    account_1.balance = "1000000000000000000".to_string(); // 1 REWA
    let mut storage_1 = std::collections::HashMap::new();
    storage_1.insert("key1".to_string(), "value1".to_string());
    let account_1 = account_1.with_storage(storage_1);
    
    let mut account_2 = SetStateAccount::from_address(
        Bech32Address::from(&account_address_2.to_address()).to_bech32_string(),
    );
    account_2.balance = "2000000000000000000".to_string(); // 2 REWA
    let mut storage_2 = std::collections::HashMap::new();
    storage_2.insert("key2".to_string(), "value2".to_string());
    let account_2 = account_2.with_storage(storage_2);

    // Save accounts to file
    let accounts = vec![account_1, account_2];
    let file_path = simulator_interact.interactor.get_state_file_path();
    let file = std::fs::File::create(&file_path).expect("Failed to create state file");
    serde_json::to_writer_pretty(file, &accounts).expect("Failed to write accounts to file");

    let set_state_response = simulator_interact
        .interactor
        .set_state_for_saved_accounts()
        .await;

    simulator_interact
        .interactor
        .generate_blocks(2u64)
        .await
        .unwrap();

    assert!(set_state_response.is_ok());

    let storage = simulator_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage.len() >= 1);

    println!("mike's storage keys in chain simulator {:#?}", storage);

    // overwrite accounts with empty
    let account_1 = SetStateAccount::from_address(
        Bech32Address::from(&account_address.to_address()).to_bech32_string(),
    );
    let account_2 = SetStateAccount::from_address(
        Bech32Address::from(&account_address_2.to_address()).to_bech32_string(),
    );

    let overwrite_vec = vec![account_1, account_2];

    simulator_interact
        .interactor
        .set_state_overwrite(overwrite_vec)
        .await
        .unwrap();

    simulator_interact
        .interactor
        .generate_blocks(2u64)
        .await
        .unwrap();

    // verify keys
    let storage_1 = simulator_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage_1.is_empty());

    println!("mike's storage keys in chain simulator {:#?}", storage_1);

    let storage_2 = simulator_interact
        .interactor
        .get_account_storage(&account_address_2.to_address())
        .await;

    assert!(storage_2.is_empty());

    println!("ivan's storage keys in chain simulator {:#?}", storage_2);
}
