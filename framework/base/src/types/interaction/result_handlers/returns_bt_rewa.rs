use crate::{
    contract_base::BlockchainWrapper,
    types::{BigUint, RHListItem, RHListItemExec, TxEnv},
};

/// Returns the amount of REWA transferred.
///
/// More precisely, it returns the sum of the REWA transfer amounts,
/// since multiple REWA transfers are possible in a multi-transfer.
///
/// It is non-exclusive, i. e. it is possible to get other tokens alongside the REWA.
pub struct ReturnsBackTransfersREWA;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfersREWA
where
    Env: TxEnv,
{
    type Returns = BigUint<Env::Api>;
}

impl<RawResult, Env, Original> RHListItemExec<RawResult, Env, Original> for ReturnsBackTransfersREWA
where
    Env: TxEnv,
{
    fn item_process_result(self, _raw_result: &RawResult) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new()
            .get_back_transfers()
            .rewa_sum()
    }
}
