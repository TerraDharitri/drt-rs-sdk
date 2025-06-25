use super::contract_impl::contract_implementation;
use crate::parse::parse_contract_trait;
use crate::validate::validate_contract;

pub fn process_contract(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let args_input = parse_macro_input!(args as syn::AttributeArgs);
	let proc_input = &parse_macro_input!(input as syn::ItemTrait);

	let contract = parse_contract_trait(args_input, proc_input);
	validate_contract(&contract);

	let contract_impl = contract_implementation(&contract, true);
	let contract_impl_ident = contract.contract_impl_name;

	let wasm_callback_endpoint = quote! {
		#[cfg(feature = "wasm-output-mode")]
		pub mod callback_endpoint {
			use super::*;

			fn new_andes_instance() -> #contract_impl_ident<numbat_wasm_node::AndesApiImpl, numbat_wasm_node::api::AndesBigInt, numbat_wasm_node::api::AndesBigUint> {
				let api = numbat_wasm_node::AndesApiImpl{};
				#contract_impl_ident::new(api)
			}

			#[no_mangle]
			#[allow(non_snake_case)]
			pub fn callBack () {
				let inst = new_andes_instance();
				inst.callback();
			}
		}
	};

	proc_macro::TokenStream::from(quote! {
		#[macro_use]
		extern crate numbat_wasm;

		#contract_impl

		#wasm_callback_endpoint
	})
}
