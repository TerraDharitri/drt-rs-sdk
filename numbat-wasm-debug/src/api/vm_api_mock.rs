use numbat_wasm::{api::VMApi, numbat_codec::TryStaticCast};

use crate::DebugApi;

impl TryStaticCast for DebugApi {}

impl VMApi for DebugApi {}
