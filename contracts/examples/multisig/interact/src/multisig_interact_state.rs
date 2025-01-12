use dharitri_sc_scenario::imports::Bech32Address;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

/// State file
const STATE_FILE: &str = "state.toml";

/// Multisig Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    multisig_address: Option<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the multisig address
    pub fn set_multisig_address(&mut self, address: Bech32Address) {
        self.multisig_address = Some(address);
    }

    pub fn current_multisig_address(&self) -> &Bech32Address {
        self.multisig_address
            .as_ref()
            .expect("no known multisig contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}
