numbat_wasm::imports!();

#[numbat_wasm::module]
pub trait ContractBaseTestModule: ContractBase {
    #[endpoint]
    fn call_contract_base_endpoint(&self) {}
}
