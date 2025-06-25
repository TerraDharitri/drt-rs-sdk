use numbat_wasm_debug::*;
use dcdt_nft_marketplace::*;

fn main() {
	let contract = DcdtNftMarketplaceImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
