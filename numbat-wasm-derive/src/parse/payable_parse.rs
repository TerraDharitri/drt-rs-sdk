use super::attributes::PayableAttribute;
use crate::model::MethodPayableMetadata;

pub fn process_payable(m: &syn::TraitItemMethod) -> MethodPayableMetadata {
	let payable_attr_opt = PayableAttribute::parse(m);
	if let Some(payable_attr) = payable_attr_opt {
		if let Some(identifier) = payable_attr.identifier {
			match identifier.as_str() {
				"REWA" => MethodPayableMetadata::Rewa,
				"*" => MethodPayableMetadata::AnyToken,
				"" => panic!("empty token name not allowed in #[payable] attribute"),
				_ => MethodPayableMetadata::SingleDcdtToken(identifier),
			}
		} else {
			eprintln!("Warning: usage of #[payable] without argument is deprecated. Replace with #[payable(\"REWA\")]. Method name: {}", m.sig.ident.to_string());
			MethodPayableMetadata::Rewa
		}
	} else {
		MethodPayableMetadata::NotPayable
	}
}
