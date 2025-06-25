use numbat_wasm::{
    api::DCDT_MULTI_TRANSFER_FUNC_NAME, numbat_codec::TopDecode, types::heap::Address,
};

use crate::{
    tx_execution::builtin_function_mocks::builtin_func_trait::BuiltinFunctionDcdtTransferInfo,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::{
    super::builtin_func_trait::BuiltinFunction,
    transfer_common::{
        execute_transfer_builtin_func, extract_transfer_info, ParsedTransferBuiltinFunCall,
        RawDcdtTransfer,
    },
};

pub struct DCDTMultiTransfer;

impl BuiltinFunction for DCDTMultiTransfer {
    fn name(&self) -> &str {
        DCDT_MULTI_TRANSFER_FUNC_NAME
    }

    fn extract_dcdt_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionDcdtTransferInfo {
        if let Ok(parsed_tx) = try_parse_input(tx_input) {
            extract_transfer_info(parsed_tx)
        } else {
            BuiltinFunctionDcdtTransferInfo::empty(tx_input)
        }
    }

    fn execute(&self, tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
        match try_parse_input(&tx_input) {
            Ok(parsed_tx) => {
                execute_transfer_builtin_func(parsed_tx, self.name(), tx_input, tx_cache)
            },
            Err(message) => {
                let err_result = TxResult::from_vm_error(message.to_string());
                (err_result, BlockchainUpdate::empty())
            },
        }
    }
}

fn try_parse_input(tx_input: &TxInput) -> Result<ParsedTransferBuiltinFunCall, &'static str> {
    if tx_input.args.len() < 2 {
        return Err("MultiDCDTNFTTransfer too few arguments");
    }
    if tx_input.to != tx_input.from {
        // TODO: not sure what the real error message would be, certainly not this
        return Err("MultiDCDTNFTTransfer expects that to == from");
    }

    let mut arg_index = 0;
    let destination_bytes = tx_input.args[arg_index].as_slice();
    let destination = Address::top_decode(destination_bytes).unwrap();
    arg_index += 1;
    let payments = usize::top_decode(tx_input.args[arg_index].as_slice()).unwrap();
    arg_index += 1;

    if tx_input.args.len() < 2 + payments * 3 {
        return Err("MultiDCDTNFTTransfer too few arguments");
    }

    let mut raw_dcdt_transfers = Vec::new();
    for _ in 0..payments {
        let token_identifier = tx_input.args[arg_index].clone();
        arg_index += 1;
        let nonce_bytes = tx_input.args[arg_index].clone();
        arg_index += 1;
        let value_bytes = tx_input.args[arg_index].clone();
        arg_index += 1;

        raw_dcdt_transfers.push(RawDcdtTransfer {
            token_identifier: token_identifier.clone(),
            nonce_bytes,
            value_bytes,
        });
    }

    let func_name = tx_input.func_name_from_arg_index(arg_index);
    arg_index += 1;
    let args = if tx_input.args.len() > arg_index {
        tx_input.args[arg_index..].to_vec()
    } else {
        Vec::new()
    };

    Ok(ParsedTransferBuiltinFunCall {
        destination,
        raw_dcdt_transfers,
        func_name,
        args,
    })
}
