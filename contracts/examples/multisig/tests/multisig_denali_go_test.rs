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
fn deploy_duplicate_bm_go() {
	numbat_wasm_debug::denali_go("denali/deploy_duplicate_bm.scen.json");
}

#[test]
fn remove_everyone_go() {
	numbat_wasm_debug::denali_go("denali/remove_everyone.scen.json");
}
