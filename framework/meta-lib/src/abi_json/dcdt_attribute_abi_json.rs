use std::collections::BTreeMap;

use dharitri_sc::abi::DcdtAttributeAbi;
use serde::{Deserialize, Serialize};

use super::{
    DcdtAttributeJson, TypeDescriptionJson, convert_type_descriptions_to_json,
    empty_type_description_container,
};

/// Represents an entire DCDT attribute ABI file. The type descriptions only show up here.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DcdtAttributeAbiJson {
    pub dcdt_attribute: DcdtAttributeJson,

    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub types: BTreeMap<String, TypeDescriptionJson>,
}

impl DcdtAttributeAbiJson {
    pub fn new(attr: &DcdtAttributeAbi) -> Self {
        DcdtAttributeAbiJson {
            dcdt_attribute: DcdtAttributeJson::from(attr),
            types: convert_type_descriptions_to_json(&attr.type_descriptions),
        }
    }
}

impl From<&DcdtAttributeAbiJson> for DcdtAttributeAbi {
    fn from(abi_json: &DcdtAttributeAbiJson) -> Self {
        DcdtAttributeAbi {
            ticker: abi_json.dcdt_attribute.ticker.clone(),
            ty: abi_json.dcdt_attribute.ty.clone(),
            type_descriptions: empty_type_description_container(), // TODO: @Laur should recursively call convert_json_to_type_descriptions
        }
    }
}
