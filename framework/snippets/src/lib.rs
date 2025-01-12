pub mod account_tool;
mod interactor;
mod multi;
pub mod network_response;

pub use env_logger;
pub use hex;
pub use interactor::*;
pub use log;
pub use multi::*;
pub use dharitri_sc_scenario::{self, dharitri_sc};
pub use dharitri_sdk as sdk_core;
pub use dharitri_sdk as sdk;

/// Imports normally needed in interactors, grouped together.
pub mod imports;

/// Backwards compatibility.
pub use crate::sdk_core::test_wallets;

#[cfg(feature = "http")]
pub type HttpInteractor = crate::InteractorBase<dharitri_sdk_http::GatewayHttpProxy>;

/// Backwards compatibility.
#[cfg(feature = "http")]
pub type Interactor = HttpInteractor;

#[cfg(feature = "dapp")]
pub type DappInteractor = crate::InteractorBase<dharitri_sdk_dapp::GatewayDappProxy>;