use numbat_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/big-float-features");

    blockchain.register_contract(
        "file:output/big-float-features.wasm",
        big_float_features::ContractBuilder,
    );
    blockchain.register_contract(
        "file:../dcdt-system-sc-mock/output/dcdt-system-sc-mock.wasm",
        dcdt_system_sc_mock::ContractBuilder,
    );

    blockchain
}

#[test]
fn big_float_new_from_big_int_rs() {
    numbat_wasm_debug::denali_rs("denali/big_float_new_from_big_int.scen.json", world());
}

#[test]
fn big_float_new_from_big_uint_rs() {
    numbat_wasm_debug::denali_rs("denali/big_float_new_from_big_uint.scen.json", world());
}

#[test]
fn big_float_new_from_frac_rs() {
    numbat_wasm_debug::denali_rs("denali/big_float_new_from_frac.scen.json", world());
}

#[test]
fn big_float_new_from_int_rs() {
    numbat_wasm_debug::denali_rs("denali/big_float_new_from_int.scen.json", world());
}

// #[test]
// fn big_float_new_from_managed_buffer_rs() {
//     numbat_wasm_debug::denali_rs(
//         "denali/big_float_new_from_managed_buffer.scen.json",
//         world(),
//     );
// }

#[test]
fn big_float_new_from_parts_rs() {
    numbat_wasm_debug::denali_rs("denali/big_float_new_from_parts.scen.json", world());
}

#[test]
fn big_float_new_from_sci_rs() {
    numbat_wasm_debug::denali_rs("denali/big_float_new_from_sci.scen.json", world());
}

#[test]
fn big_float_operators_rs() {
    numbat_wasm_debug::denali_rs("denali/big_float_operators.scen.json", world());
}

// #[test]
// fn big_float_operator_checks_rs() {
//     numbat_wasm_debug::denali_rs("denali/big_float_operator_checks.scen.json", world());
// }
