use super::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

pub enum CheckStorageRaw {
	Star,
	Equal(BTreeMap<String, CheckBytesValueRaw>),
}

impl CheckStorageRaw {
	pub fn is_star(&self) -> bool {
		matches!(self, CheckStorageRaw::Star)
	}
}

impl Default for CheckStorageRaw {
	fn default() -> Self {
		CheckStorageRaw::Equal(BTreeMap::new())
	}
}

impl Serialize for CheckStorageRaw {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			CheckStorageRaw::Star => serializer.serialize_str("*"),
			CheckStorageRaw::Equal(m) => {
				let mut map = serializer.serialize_map(Some(m.len()))?;
				for (k, v) in m {
					map.serialize_entry(k, v)?;
				}
				map.end()
			},
		}
	}
}

struct CheckStorageRawVisitor;

impl<'de> Visitor<'de> for CheckStorageRawVisitor {
	type Value = CheckStorageRaw;

	// Format a message stating what data this Visitor expects to receive.
	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("serialized object JSON representation of log check")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		if value == "*" {
			Ok(CheckStorageRaw::Star)
		} else {
			Err(de::Error::custom("only '*' allowed as logs string value"))
		}
	}

	fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut map = BTreeMap::<String, CheckBytesValueRaw>::new();

		// While there are entries remaining in the input, add them
		// into our map.
		while let Some((key, value)) = access.next_entry()? {
			map.insert(key, value);
		}

		Ok(CheckStorageRaw::Equal(map))
	}
}

impl<'de> Deserialize<'de> for CheckStorageRaw {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(CheckStorageRawVisitor)
	}
}

pub enum CheckDcdtRaw {
	Unspecified,
	Star,
	Equal(BTreeMap<String, CheckBytesValueRaw>),
}

impl CheckDcdtRaw {
	pub fn is_star(&self) -> bool {
		matches!(self, CheckDcdtRaw::Star)
	}

	pub fn is_unspecified(&self) -> bool {
		matches!(self, CheckDcdtRaw::Unspecified)
	}
}

impl Default for CheckDcdtRaw {
	fn default() -> Self {
		CheckDcdtRaw::Unspecified
	}
}

impl Serialize for CheckDcdtRaw {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			CheckDcdtRaw::Unspecified => {
				// empty map, just in case
				// won't get serialized anyway
				let map = serializer.serialize_map(Some(0))?;
				map.end()
			},
			CheckDcdtRaw::Star => serializer.serialize_str("*"),
			CheckDcdtRaw::Equal(m) => {
				let mut map = serializer.serialize_map(Some(m.len()))?;
				for (k, v) in m {
					map.serialize_entry(k, v)?;
				}
				map.end()
			},
		}
	}
}

struct CheckDcdtRawVisitor;

impl<'de> Visitor<'de> for CheckDcdtRawVisitor {
	type Value = CheckDcdtRaw;

	// Format a message stating what data this Visitor expects to receive.
	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("serialized object JSON representation of dcdt check")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		if value == "*" {
			Ok(CheckDcdtRaw::Star)
		} else {
			Err(de::Error::custom("only '*' allowed as dcdt string value"))
		}
	}

	fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut map = BTreeMap::<String, CheckBytesValueRaw>::new();

		// While there are entries remaining in the input, add them
		// into our map.
		while let Some((key, value)) = access.next_entry()? {
			map.insert(key, value);
		}

		Ok(CheckDcdtRaw::Equal(map))
	}
}

impl<'de> Deserialize<'de> for CheckDcdtRaw {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(CheckDcdtRawVisitor)
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAccountRaw {
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub comment: Option<String>,

	#[serde(default)]
	#[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
	pub nonce: CheckBytesValueRaw,

	#[serde(default)]
	#[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
	pub balance: CheckBytesValueRaw,

	#[serde(default)]
	#[serde(skip_serializing_if = "CheckDcdtRaw::is_unspecified")]
	pub dcdt: CheckDcdtRaw,

	#[serde(default)]
	#[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
	pub username: CheckBytesValueRaw,

	#[serde(default)]
	pub storage: CheckStorageRaw,

	#[serde(default)]
	#[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
	pub code: CheckBytesValueRaw,

	#[serde(default)]
	#[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
	pub async_call_data: CheckBytesValueRaw,
}

pub enum CheckAccountRawOrNothing {
	Some(CheckAccountRaw),
	Nothing,
}

struct CheckAccountRawOrNothingVisitor;

impl<'de> Visitor<'de> for CheckAccountRawOrNothingVisitor {
	type Value = CheckAccountRawOrNothing;

	// Format a message stating what data this Visitor expects to receive.
	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("CheckAccountRaw or nothing")
	}

	fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		Ok(CheckAccountRawOrNothing::Nothing)
	}

	fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		Ok(CheckAccountRawOrNothing::Some(Deserialize::deserialize(
			de::value::MapAccessDeserializer::new(map),
		)?))
	}
}

impl<'de> Deserialize<'de> for CheckAccountRawOrNothing {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(CheckAccountRawOrNothingVisitor)
	}
}

pub struct CheckAccountsRaw {
	pub other_accounts_allowed: bool,
	pub accounts: BTreeMap<String, CheckAccountRaw>,
}

impl Serialize for CheckAccountsRaw {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut map = serializer.serialize_map(Some(self.accounts.len()))?;
		for (k, v) in self.accounts.iter() {
			map.serialize_entry(k, v)?;
		}
		if self.other_accounts_allowed {
			map.serialize_entry("+", "")?;
		}
		map.end()
	}
}

struct CheckAccountRawsVisitor;

impl<'de> Visitor<'de> for CheckAccountRawsVisitor {
	type Value = CheckAccountsRaw;

	// Format a message stating what data this Visitor expects to receive.
	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("serialized CheckAccountsRaw")
	}

	fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut accounts = BTreeMap::<String, CheckAccountRaw>::new();
		let mut other_accounts_allowed = false;

		// While there are entries remaining in the input, add them
		// into our map.
		while let Some((key, value)) = access.next_entry()? {
			if key == "+" {
				other_accounts_allowed = true;
			} else if let CheckAccountRawOrNothing::Some(check_account) = value {
				accounts.insert(key, check_account);
			} else {
				return Err(de::Error::custom("invalid CheckAccountRaw"));
			}
		}

		Ok(CheckAccountsRaw {
			other_accounts_allowed,
			accounts,
		})
	}
}

impl<'de> Deserialize<'de> for CheckAccountsRaw {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(CheckAccountRawsVisitor)
	}
}
