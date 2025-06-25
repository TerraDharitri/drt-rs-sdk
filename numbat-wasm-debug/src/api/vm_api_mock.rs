use numbat_wasm::{api::VMApi, numbat_codec::TryStaticCast};

use crate::TxContext;

impl TryStaticCast for TxContext {}

impl VMApi for TxContext {}
