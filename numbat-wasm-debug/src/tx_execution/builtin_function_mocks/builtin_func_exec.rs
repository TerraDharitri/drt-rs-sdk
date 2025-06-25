use std::rc::Rc;

use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::{
    builtin_func_map::BuiltinFunctionMap,
    builtin_func_role_check_wrapper::BuiltinFunctionRoleCheckWrapper,
    builtin_func_trait::BuiltinFunction,
    dcdt_nft::{
        DCDTLocalBurn, DCDTLocalMint, DCDTNftAddQuantity, DCDTNftAddUri, DCDTNftBurn,
        DCDTNftCreate, DCDTNftUpdateAttributes,
    },
    general::{ChangeOwner, ClaimDeveloperRewards, SetUsername, UpgradeContract},
    transfer::{DCDTMultiTransfer, DCDTNftTransfer, DCDTTransfer},
};

const DCDT_ROLE_LOCAL_MINT: &str = "DCDTRoleLocalMint";
const DCDT_ROLE_LOCAL_BURN: &str = "DCDTRoleLocalBurn";
const DCDT_ROLE_NFT_CREATE: &str = "DCDTRoleNFTCreate";
const DCDT_ROLE_NFT_ADD_QUANTITY: &str = "DCDTRoleNFTAddQuantity";
const DCDT_ROLE_NFT_BURN: &str = "DCDTRoleNFTBurn";
const DCDT_ROLE_NFT_ADD_URI: &str = "DCDTRoleNFTAddURI";
const DCDT_ROLE_NFT_UPDATE_ATTRIBUTES: &str = "DCDTRoleNFTUpdateAttributes";

fn builtin_function_impls() -> Vec<Box<dyn BuiltinFunction>> {
    vec![
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCDT_ROLE_LOCAL_MINT,
            Box::new(DCDTLocalMint),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCDT_ROLE_LOCAL_BURN,
            Box::new(DCDTLocalBurn),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCDT_ROLE_NFT_CREATE,
            Box::new(DCDTNftCreate),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCDT_ROLE_NFT_ADD_QUANTITY,
            Box::new(DCDTNftAddQuantity),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCDT_ROLE_NFT_BURN,
            Box::new(DCDTNftBurn),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCDT_ROLE_NFT_ADD_URI,
            Box::new(DCDTNftAddUri),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCDT_ROLE_NFT_UPDATE_ATTRIBUTES,
            Box::new(DCDTNftUpdateAttributes),
        )),
        Box::new(DCDTMultiTransfer),
        Box::new(DCDTNftTransfer),
        Box::new(DCDTTransfer),
        Box::new(ChangeOwner),
        Box::new(ClaimDeveloperRewards),
        Box::new(SetUsername),
        Box::new(UpgradeContract),
    ]
}

pub fn init_builtin_functions() -> BuiltinFunctionMap {
    BuiltinFunctionMap::init(builtin_function_impls())
}

pub fn execute_builtin_function_or_default(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    let builtin_functions = Rc::clone(&tx_cache.blockchain_ref().builtin_functions);
    if let Some(builtin_func) = builtin_functions.get(&tx_input.func_name) {
        builtin_func.execute(tx_input, tx_cache)
    } else {
        default_execution(tx_input, tx_cache)
    }
}
