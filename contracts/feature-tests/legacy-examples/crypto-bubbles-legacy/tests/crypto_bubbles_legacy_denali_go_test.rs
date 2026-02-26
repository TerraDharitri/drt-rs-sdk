#[test]
fn balanceof_go() {
    numbat_wasm_debug::denali_go("denali/balanceOf.scen.json");
}

#[test]
fn create_go() {
    numbat_wasm_debug::denali_go("denali/create.scen.json");
}

#[test]
fn exceptions_go() {
    numbat_wasm_debug::denali_go("denali/exceptions.scen.json");
}

#[test]
fn joingame_go() {
    numbat_wasm_debug::denali_go("denali/joinGame.scen.json");
}

#[test]
fn rewardandsendtowallet_go() {
    numbat_wasm_debug::denali_go("denali/rewardAndSendToWallet.scen.json");
}

#[test]
fn rewardwinner_go() {
    numbat_wasm_debug::denali_go("denali/rewardWinner.scen.json");
}

#[test]
fn rewardwinner_last_go() {
    numbat_wasm_debug::denali_go("denali/rewardWinner_Last.scen.json");
}

#[test]
fn topup_ok_go() {
    numbat_wasm_debug::denali_go("denali/topUp_ok.scen.json");
}

#[test]
fn topup_withdraw_go() {
    numbat_wasm_debug::denali_go("denali/topUp_withdraw.scen.json");
}

#[test]
fn withdraw_ok_go() {
    numbat_wasm_debug::denali_go("denali/withdraw_Ok.scen.json");
}

#[test]
fn withdraw_toomuch_go() {
    numbat_wasm_debug::denali_go("denali/withdraw_TooMuch.scen.json");
}
