use std::sync::Arc;

#[allow(unused_imports)]
use dharitri_sc::imports::*;

pub struct Config {
    pub settings: Arc<Vec<u8>>,
    pub allowed: bool,
}

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[dharitri_sc::contract]
pub trait StdContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
