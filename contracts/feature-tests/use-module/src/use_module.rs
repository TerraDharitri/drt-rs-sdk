#![no_std]

mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;

numbat_wasm::imports!();

use numbat_wasm_module_features::feature_guard;

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - DnsModule
/// - FeaturesModule
/// - DcdtModule
/// - GovernanceModule
/// - PauseModule
#[numbat_wasm::contract]
pub trait UseModule:
    internal_mod_a::InternalModuleA
    + internal_mod_b::InternalModuleB
    + internal_mod_c::InternalModuleC
    + numbat_wasm_module_dns::DnsModule
    + numbat_wasm_module_dcdt::DcdtModule
    + numbat_wasm_module_features::FeaturesModule
    + numbat_wasm_module_governance::GovernanceModule
    + numbat_wasm_module_governance::governance_configurable::GovernanceConfigurablePropertiesModule
    + numbat_wasm_module_pause::PauseModule
{
    #[init]
    fn init(&self) {}

    /// Validates that the "featureName" feature is on.
    /// Uses the `feature_guard!` macro.
    #[endpoint(checkFeatureGuard)]
    fn check_feature_guard(&self) -> SCResult<()> {
        feature_guard!(self, b"featureName", true);
        Ok(())
    }

    #[endpoint(checkPause)]
    fn check_pause(&self) -> SCResult<bool> {
        Ok(self.is_paused())
    }
}
