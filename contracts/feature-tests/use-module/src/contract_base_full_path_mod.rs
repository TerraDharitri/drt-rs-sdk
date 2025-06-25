numbat_wasm::imports!();

#[numbat_wasm::module]
pub trait ContractBaseFullPathTestModule: numbat_wasm::contract_base::ContractBase {
    #[endpoint]
    fn call_contract_base_full_path_endpoint(&self) {}
}
