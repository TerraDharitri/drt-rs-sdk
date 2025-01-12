use core::marker::PhantomData;

use crate::{
    api::{
        const_handles, use_raw_handle, BigIntApiImpl, BlockchainApi, BlockchainApiImpl, ErrorApi,
        ErrorApiImpl, HandleConstraints, ManagedBufferApiImpl, ManagedTypeApi, ManagedTypeApiImpl,
        StaticVarApiImpl, StorageReadApi, StorageReadApiImpl,
    },
    codec::TopDecode,
    err_msg::{ONLY_OWNER_CALLER, ONLY_USER_ACCOUNT_CALLER},
    storage::{self},
    types::{
        BackTransfers, BigUint, CodeMetadata, MoaOrDcdtTokenIdentifier, DcdtLocalRoleFlags,
        DcdtTokenData, DcdtTokenType, ManagedAddress, ManagedBuffer, ManagedByteArray,
        ManagedRefMut, ManagedType, ManagedVec, TokenIdentifier,
    },
};

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
#[derive(Default)]
pub struct BlockchainWrapper<A>
where
    A: BlockchainApi + ManagedTypeApi + ErrorApi,
{
    _phantom: PhantomData<A>,
}

impl<A> BlockchainWrapper<A>
where
    A: BlockchainApi + ManagedTypeApi + ErrorApi,
{
    pub fn new() -> Self {
        BlockchainWrapper {
            _phantom: PhantomData,
        }
    }

    #[deprecated(since = "0.41.0", note = "Please use method `get_caller` instead.")]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_caller_legacy(&self) -> crate::types::Address {
        A::blockchain_api_impl().get_caller_legacy()
    }

    #[inline]
    pub fn get_caller(&self) -> ManagedAddress<A> {
        unsafe {
            let result = ManagedAddress::new_uninit();
            A::blockchain_api_impl().load_caller_managed(result.get_handle());
            result
        }
    }

    #[deprecated(since = "0.41.0", note = "Please use method `get_sc_address` instead.")]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_sc_address_legacy(&self) -> crate::types::Address {
        A::blockchain_api_impl().get_sc_address_legacy()
    }

    #[inline]
    pub fn get_sc_address(&self) -> ManagedAddress<A> {
        unsafe {
            let result = ManagedAddress::new_uninit();
            A::blockchain_api_impl().load_sc_address_managed(result.get_handle());
            result
        }
    }

    #[inline]
    pub fn get_owner_address(&self) -> ManagedAddress<A> {
        unsafe {
            let result = ManagedAddress::new_uninit();
            A::blockchain_api_impl().load_owner_address_managed(result.get_handle());
            result
        }
    }

    pub fn check_caller_is_owner(&self) {
        if self.get_owner_address() != self.get_caller() {
            A::error_api_impl().signal_error(ONLY_OWNER_CALLER.as_bytes());
        }
    }

    pub fn check_caller_is_user_account(&self) {
        let mbuf_temp_1: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::blockchain_api_impl().load_caller_managed(mbuf_temp_1.clone());
        if A::blockchain_api_impl().is_smart_contract(mbuf_temp_1) {
            A::error_api_impl().signal_error(ONLY_USER_ACCOUNT_CALLER.as_bytes());
        }
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `get_shard_of_address` instead."
    )]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_shard_of_address_legacy(&self, address: &crate::types::Address) -> u32 {
        A::blockchain_api_impl().get_shard_of_address_legacy(address)
    }

    #[inline]
    pub fn get_shard_of_address(&self, address: &ManagedAddress<A>) -> u32 {
        A::blockchain_api_impl().get_shard_of_address(address.get_handle())
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `is_smart_contract` instead."
    )]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn is_smart_contract_legacy(&self, address: &crate::types::Address) -> bool {
        A::blockchain_api_impl().is_smart_contract_legacy(address)
    }

    #[inline]
    pub fn is_smart_contract(&self, address: &ManagedAddress<A>) -> bool {
        A::blockchain_api_impl().is_smart_contract(address.get_handle())
    }

    #[deprecated(since = "0.41.0", note = "Please use method `get_balance` instead.")]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_balance_legacy(&self, address: &crate::types::Address) -> BigUint<A> {
        unsafe {
            let result = BigUint::new_uninit();
            A::blockchain_api_impl().load_balance_legacy(result.get_handle(), address);
            result
        }
    }

    #[inline]
    pub fn get_balance(&self, address: &ManagedAddress<A>) -> BigUint<A> {
        unsafe {
            let result = BigUint::new_uninit();
            A::blockchain_api_impl().load_balance(result.get_handle(), address.get_handle());
            result
        }
    }

    #[inline]
    pub fn get_code_metadata(&self, address: &ManagedAddress<A>) -> CodeMetadata {
        let mbuf_temp_1: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::blockchain_api_impl()
            .managed_get_code_metadata(address.get_handle(), mbuf_temp_1.clone());
        let mut buffer = [0u8; 2];
        unsafe {
            ManagedRefMut::<'static, A, ManagedBuffer<A>>::wrap_handle(mbuf_temp_1)
                .load_to_byte_array(&mut buffer);
        }
        CodeMetadata::from(buffer)
    }

    #[inline]
    pub fn is_builtin_function(&self, function_name: &ManagedBuffer<A>) -> bool {
        A::blockchain_api_impl().managed_is_builtin_function(function_name.get_handle())
    }

    #[inline]
    pub fn get_sc_balance(&self, token: &MoaOrDcdtTokenIdentifier<A>, nonce: u64) -> BigUint<A> {
        token.map_ref_or_else(
            (),
            |()| self.get_balance(&self.get_sc_address()),
            |(), token_identifier| {
                self.get_dcdt_balance(&self.get_sc_address(), token_identifier, nonce)
            },
        )
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `get_state_root_hash` instead."
    )]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_state_root_hash_legacy(&self) -> crate::types::H256 {
        self.get_state_root_hash().to_byte_array().into()
    }

    #[inline]
    pub fn get_state_root_hash(&self) -> ManagedByteArray<A, 32> {
        unsafe {
            let result = ManagedByteArray::new_uninit();
            A::blockchain_api_impl().load_state_root_hash_managed(result.get_handle());
            result
        }
    }

    #[deprecated(since = "0.41.0", note = "Please use method `get_tx_hash` instead.")]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_tx_hash_legacy(&self) -> crate::types::H256 {
        A::blockchain_api_impl().get_tx_hash_legacy()
    }

    #[inline]
    pub fn get_tx_hash(&self) -> ManagedByteArray<A, 32> {
        unsafe {
            let result = ManagedByteArray::new_uninit();
            A::blockchain_api_impl().load_tx_hash_managed(result.get_handle());
            result
        }
    }

    #[inline]
    pub fn get_gas_left(&self) -> u64 {
        A::blockchain_api_impl().get_gas_left()
    }

    #[inline]
    pub fn get_block_timestamp(&self) -> u64 {
        A::blockchain_api_impl().get_block_timestamp()
    }

    #[inline]
    pub fn get_block_nonce(&self) -> u64 {
        A::blockchain_api_impl().get_block_nonce()
    }

    #[inline]
    pub fn get_block_round(&self) -> u64 {
        A::blockchain_api_impl().get_block_round()
    }

    #[inline]
    pub fn get_block_epoch(&self) -> u64 {
        A::blockchain_api_impl().get_block_epoch()
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `get_block_random_seed` instead."
    )]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_block_random_seed_legacy(&self) -> crate::types::Box<[u8; 48]> {
        crate::types::Box::new(self.get_block_random_seed().to_byte_array())
    }

    #[inline]
    pub fn get_block_random_seed(&self) -> ManagedByteArray<A, 48> {
        unsafe {
            let result = ManagedByteArray::new_uninit();
            A::blockchain_api_impl().load_block_random_seed_managed(result.get_handle());
            result
        }
    }

    #[inline]
    pub fn get_prev_block_timestamp(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_timestamp()
    }

    #[inline]
    pub fn get_prev_block_nonce(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_nonce()
    }

    #[inline]
    pub fn get_prev_block_round(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_round()
    }

    #[inline]
    pub fn get_prev_block_epoch(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_epoch()
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `get_prev_block_random_seed` instead."
    )]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_prev_block_random_seed_legacy(&self) -> crate::types::Box<[u8; 48]> {
        A::blockchain_api_impl().get_prev_block_random_seed_legacy()
    }

    #[inline]
    pub fn get_prev_block_random_seed(&self) -> ManagedByteArray<A, 48> {
        unsafe {
            let result = ManagedByteArray::new_uninit();
            A::blockchain_api_impl().load_prev_block_random_seed_managed(result.get_handle());
            result
        }
    }

    #[inline]
    pub fn get_current_dcdt_nft_nonce(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
    ) -> u64 {
        A::blockchain_api_impl()
            .get_current_dcdt_nft_nonce(address.get_handle(), token_id.get_handle())
    }

    #[inline]
    pub fn get_dcdt_balance(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
    ) -> BigUint<A> {
        unsafe {
            let result = BigUint::new_uninit();
            A::blockchain_api_impl().load_dcdt_balance(
                address.get_handle(),
                token_id.get_handle(),
                nonce,
                result.get_handle(),
            );
            result
        }
    }

    pub fn get_dcdt_token_data(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
    ) -> DcdtTokenData<A> {
        // initializing outputs
        // the current version of VM does not set/overwrite them if the token is missing,
        // which is why we need to initialize them explicitly
        let managed_api_impl = A::managed_type_impl();
        let value_handle = managed_api_impl.bi_new_zero();
        let properties_handle = managed_api_impl.mb_new_empty(); // TODO: replace with const_handles::MBUF_TEMPORARY_1 after VM fix
        let hash_handle = managed_api_impl.mb_new_empty();
        let name_handle = managed_api_impl.mb_new_empty();
        let attributes_handle = managed_api_impl.mb_new_empty();
        let creator_handle = managed_api_impl.mb_new_empty();
        let royalties_handle = managed_api_impl.bi_new_zero();
        let uris_handle = managed_api_impl.mb_new_empty();

        A::blockchain_api_impl().managed_get_dcdt_token_data(
            address.get_handle().get_raw_handle(),
            token_id.get_handle().get_raw_handle(),
            nonce,
            value_handle.get_raw_handle(),
            properties_handle.get_raw_handle(),
            hash_handle.get_raw_handle(),
            name_handle.get_raw_handle(),
            attributes_handle.get_raw_handle(),
            creator_handle.get_raw_handle(),
            royalties_handle.get_raw_handle(),
            uris_handle.get_raw_handle(),
        );

        let token_type = if nonce == 0 {
            DcdtTokenType::Fungible
        } else {
            DcdtTokenType::NonFungible
        };

        if managed_api_impl.mb_len(creator_handle.clone()) == 0 {
            managed_api_impl.mb_overwrite(creator_handle.clone(), &[0u8; 32][..]);
        }

        // here we trust Andes that it always gives us a properties buffer of length 2
        let mut properties_bytes = [0u8; 2];
        let _ = managed_api_impl.mb_load_slice(properties_handle, 0, &mut properties_bytes[..]);
        let frozen = dcdt_is_frozen(&properties_bytes);

        unsafe {
            DcdtTokenData {
                token_type,
                amount: BigUint::from_raw_handle(value_handle.get_raw_handle()),
                frozen,
                hash: ManagedBuffer::from_raw_handle(hash_handle.get_raw_handle()),
                name: ManagedBuffer::from_raw_handle(name_handle.get_raw_handle()),
                attributes: ManagedBuffer::from_raw_handle(attributes_handle.get_raw_handle()),
                creator: ManagedAddress::from_raw_handle(creator_handle.get_raw_handle()),
                royalties: BigUint::from_raw_handle(royalties_handle.get_raw_handle()),
                uris: ManagedVec::from_raw_handle(uris_handle.get_raw_handle()),
            }
        }
    }

    /// Retrieves back-transfers from the VM, after a contract call.
    ///
    /// Works after:
    /// - synchronous calls
    /// - asynchronous calls too, in callbacks.
    pub fn get_back_transfers(&self) -> BackTransfers<A> {
        let dcdt_transfer_value_handle: A::BigIntHandle =
            use_raw_handle(A::static_var_api_impl().next_handle());
        let call_value_handle: A::BigIntHandle =
            use_raw_handle(A::static_var_api_impl().next_handle());

        A::blockchain_api_impl().managed_get_back_transfers(
            dcdt_transfer_value_handle.get_raw_handle(),
            call_value_handle.get_raw_handle(),
        );

        unsafe {
            BackTransfers {
                total_moa_amount: BigUint::from_raw_handle(call_value_handle.get_raw_handle()),
                dcdt_payments: ManagedVec::from_raw_handle(
                    dcdt_transfer_value_handle.get_raw_handle(),
                ),
            }
        }
    }

    /// Retrieves and deserializes token attributes from the SC account, with given token identifier and nonce.
    pub fn get_token_attributes<T: TopDecode>(
        &self,
        token_id: &TokenIdentifier<A>,
        token_nonce: u64,
    ) -> T {
        let own_sc_address = self.get_sc_address();
        let token_data = self.get_dcdt_token_data(&own_sc_address, token_id, token_nonce);
        token_data.decode_attributes()
    }

    #[inline]
    pub fn is_dcdt_frozen(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
    ) -> bool {
        A::blockchain_api_impl().check_dcdt_frozen(
            address.get_handle(),
            token_id.get_handle(),
            nonce,
        )
    }

    #[inline]
    pub fn is_dcdt_paused(&self, token_id: &TokenIdentifier<A>) -> bool {
        A::blockchain_api_impl().check_dcdt_paused(token_id.get_handle())
    }

    #[inline]
    pub fn is_dcdt_limited_transfer(&self, token_id: &TokenIdentifier<A>) -> bool {
        A::blockchain_api_impl().check_dcdt_limited_transfer(token_id.get_handle())
    }

    #[inline]
    pub fn get_dcdt_local_roles(&self, token_id: &TokenIdentifier<A>) -> DcdtLocalRoleFlags {
        A::blockchain_api_impl().load_dcdt_local_roles(token_id.get_handle())
    }
}

impl<A> BlockchainWrapper<A>
where
    A: BlockchainApi + StorageReadApi + ManagedTypeApi + ErrorApi,
{
    /// Retrieves validator rewards, as set by the protocol.
    #[inline]
    pub fn get_cumulated_validator_rewards(&self) -> BigUint<A> {
        let temp_handle_1: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        let temp_handle_2: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_2);

        // prepare key
        A::managed_type_impl().mb_overwrite(
            temp_handle_1.clone(),
            storage::protected_keys::NUMBAT_REWARD_KEY,
        );

        // load value
        A::storage_read_api_impl()
            .storage_load_managed_buffer_raw(temp_handle_1, temp_handle_2.clone());

        // convert value to BigUint
        let result = unsafe { BigUint::new_uninit() };
        A::managed_type_impl().mb_to_big_int_unsigned(temp_handle_2, result.get_handle());
        result
    }
}

fn dcdt_is_frozen(properties_bytes: &[u8; 2]) -> bool {
    properties_bytes[0] > 0 // token is frozen if the first byte is 1
}