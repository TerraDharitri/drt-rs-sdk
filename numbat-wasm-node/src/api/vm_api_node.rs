use crate::AndesApiImpl;
use numbat_wasm::{api::VMApi, numbat_codec::TryStaticCast};

impl TryStaticCast for AndesApiImpl {}

impl VMApi for AndesApiImpl {}
