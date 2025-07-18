use dharitri_sc_snippets::{account_tool, imports::GatewayHttpProxy};

use crate::cli::AccountArgs;

/// Interprets arguments and call the account tool from `dharitri_sc_snippets`.
pub async fn retrieve_address(args: &AccountArgs) {
    // validated before, can unwrap directly
    let api_string = args.api.clone().unwrap();
    account_tool::print_account_as_scenario_set_state(
        GatewayHttpProxy::new(api_string),
        args.address.to_string(),
    )
    .await;
}
