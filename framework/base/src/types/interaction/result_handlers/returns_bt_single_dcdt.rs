use crate::{
    contract_base::BlockchainWrapper,
    types::{DcdtTokenPayment, RHListItem, RHListItemExec, TxEnv},
};

/// Requests a single DCDT to be returned as back-transfer. Will fail otherwise.
pub struct ReturnsBackTransfersSingleDCDT;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfersSingleDCDT
where
    Env: TxEnv,
{
    type Returns = DcdtTokenPayment<Env::Api>;
}

impl<RawResult, Env, Original> RHListItemExec<RawResult, Env, Original>
    for ReturnsBackTransfersSingleDCDT
where
    Env: TxEnv,
{
    fn item_process_result(self, _raw_result: &RawResult) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new()
            .get_back_transfers()
            .to_single_dcdt()
    }
}
