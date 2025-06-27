mod install_scenario_go;
mod install_wasm_tools;
mod system_info;

use crate::cli_args::{
    InstallArgs, InstallCommand, InstallDrtGoScenarioArgs, InstallWasm32Args, InstallWasmOptArgs,
};

use self::install_scenario_go::ScenarioGoInstaller;

pub fn install(args: &InstallArgs) {
    let command = args
        .command
        .as_ref()
        .expect("command expected after `install`");

    match command {
        InstallCommand::All => {
            install_scenario_go(&InstallDrtGoScenarioArgs::default());
            install_wasm32(&InstallWasm32Args::default());
            install_wasm_opt(&InstallWasmOptArgs::default());
        },
        InstallCommand::DrtGoScenario(sg_args) => install_scenario_go(sg_args),
        InstallCommand::Wasm32(wam32_args) => install_wasm32(wam32_args),
        InstallCommand::WasmOpt(wasm_opt_args) => install_wasm_opt(wasm_opt_args),
    }
}

fn install_scenario_go(sg_args: &InstallDrtGoScenarioArgs) {
    ScenarioGoInstaller::new(sg_args.tag.clone()).install();
}

fn install_wasm32(_wasm32_args: &InstallWasm32Args) {
    install_wasm_tools::install_wasm32_target();
}

fn install_wasm_opt(_wasm_opt_args: &InstallWasmOptArgs) {
    install_wasm_tools::install_wasm_opt();
}
