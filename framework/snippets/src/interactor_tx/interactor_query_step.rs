use dharitri_sc_scenario::{
    dharitri_sc::{tuple_util::NestedTupleFlatten, types::RHListExec},
    scenario::tx_to_step::StepWrapper,
    scenario_model::{ScQueryStep, TxResponse},
};

use super::InteractorQueryEnv;

pub struct InteractorQueryStep<'w, RH>
where
    RH: RHListExec<TxResponse, InteractorQueryEnv<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub(crate) step_wrapper: StepWrapper<InteractorQueryEnv<'w>, ScQueryStep, RH>,
}
