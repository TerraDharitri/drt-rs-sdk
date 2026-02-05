use crate::{
    host::{
        context::TxTokenTransfer,
        vm_hooks::{VMHooksContext, vh_early_exit::early_exit_vm_error},
    },
    types::RawHandle,
    vm_err_msg,
};
use dharitri_chain_core::REWA_000000_TOKEN_IDENTIFIER;
use dharitri_vm_executor::VMHooksEarlyExit;
use num_traits::Zero;

use super::VMHooksHandler;

impl<C: VMHooksContext> VMHooksHandler<C> {
    pub fn check_not_payable(&mut self) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_call_value)?;

        if self.context.input_ref().rewa_value > num_bigint::BigUint::zero() {
            return Err(early_exit_vm_error(vm_err_msg::NON_PAYABLE_FUNC_REWA));
        }
        if self.dcdt_num_transfers() > 0 {
            return Err(early_exit_vm_error(vm_err_msg::NON_PAYABLE_FUNC_DCDT));
        }
        Ok(())
    }

    pub fn load_rewa_value(&mut self, dest: RawHandle) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        let value = self.context.input_ref().received_rewa().clone();
        self.context.m_types_lock().bi_overwrite(dest, value.into());

        Ok(())
    }

    pub fn load_all_dcdt_transfers(
        &mut self,
        dest_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let num_bytes_copied = self
            .context
            .m_types_lock()
            .mb_set_vec_of_dcdt_payments(dest_handle, self.context.input_ref().received_dcdt());
        self.use_gas_for_data_copy(num_bytes_copied)?;

        Ok(())
    }

    pub fn load_all_transfers(&self, dest_handle: RawHandle) -> Result<(), VMHooksEarlyExit> {
        let direct_rewa_value = self.context.input_ref().received_rewa().clone();
        let transfers = if !direct_rewa_value.is_zero() {
            vec![TxTokenTransfer {
                token_identifier: REWA_000000_TOKEN_IDENTIFIER.as_bytes().to_vec(),
                nonce: 0,
                value: direct_rewa_value,
            }]
        } else {
            self.context.input_ref().received_dcdt().to_owned()
        };
        self.context
            .m_types_lock()
            .mb_set_vec_of_dcdt_payments(dest_handle, &transfers);
        Ok(())
    }

    pub fn dcdt_num_transfers(&mut self) -> usize {
        self.context.input_ref().received_dcdt().len()
    }
}
