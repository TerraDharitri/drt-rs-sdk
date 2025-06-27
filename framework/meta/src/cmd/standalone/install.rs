mod install_scenario_go;
mod system_info;

use crate::cli_args::{InstallArgs, InstallCommand, InstallDrtGoScenarioArgs};

use self::install_scenario_go::ScenarioGoInstaller;

pub fn install(args: &InstallArgs) {
    let command = args
        .command
        .as_ref()
        .expect("command expected after `install`");

    match command {
        InstallCommand::All => {
            install_scenario_go(&InstallDrtGoScenarioArgs::default());
        },
        InstallCommand::DrtGoScenario(sg_args) => install_scenario_go(sg_args),
    }
}

fn install_scenario_go(sg_args: &InstallDrtGoScenarioArgs) {
    ScenarioGoInstaller::new(sg_args.tag.clone()).install();
}
