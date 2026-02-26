#[test]
fn call_other_shard_1_go() {
    numbat_wasm_debug::denali_go("denali/call_other_shard-1.scen.json");
}

#[test]
fn call_other_shard_2_go() {
    numbat_wasm_debug::denali_go("denali/call_other_shard-2.scen.json");
}

// #[test]
// fn call_other_shard_insufficient_gas_go() {
//     numbat_wasm_debug::denali_go("denali/call_other_shard-insufficient-gas.scen.json");
// }

#[test]
fn changeboard_go() {
    numbat_wasm_debug::denali_go("denali/changeBoard.scen.json");
}

#[test]
fn changequorum_go() {
    numbat_wasm_debug::denali_go("denali/changeQuorum.scen.json");
}

#[test]
fn changequorum_toobig_go() {
    numbat_wasm_debug::denali_go("denali/changeQuorum_tooBig.scen.json");
}

#[test]
fn deployadder_err_go() {
    numbat_wasm_debug::denali_go("denali/deployAdder_err.scen.json");
}

#[test]
fn deployadder_then_call_go() {
    numbat_wasm_debug::denali_go("denali/deployAdder_then_call.scen.json");
}

#[test]
fn deployfactorial_go() {
    numbat_wasm_debug::denali_go("denali/deployFactorial.scen.json");
}

#[test]
fn deployothermultisig_go() {
    numbat_wasm_debug::denali_go("denali/deployOtherMultisig.scen.json");
}

#[test]
fn deploy_duplicate_bm_go() {
    numbat_wasm_debug::denali_go("denali/deploy_duplicate_bm.scen.json");
}

#[test]
fn remove_everyone_go() {
    numbat_wasm_debug::denali_go("denali/remove_everyone.scen.json");
}

// TODO: investigate gas issue
// #[test]
// fn senddcdt_go() {
//     numbat_wasm_debug::denali_go("denali/sendDcdt.scen.json");
// }

#[test]
fn upgrade_go() {
    numbat_wasm_debug::denali_go("denali/upgrade.scen.json");
}

#[test]
fn upgrade_from_source_go() {
    numbat_wasm_debug::denali_go("denali/upgrade_from_source.scen.json");
}
