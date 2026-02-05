mod rewa_or_dcdt_token_identifier;
mod rewa_or_dcdt_token_payment;
mod rewa_or_dcdt_token_payment_refs;
mod rewa_or_multi_dcdt_payment;
mod dcdt_token_data;
mod dcdt_token_identifier;
mod dcdt_token_payment;
mod multi_rewa_or_dcdt_token_payment;
mod multi_transfer_marker;
mod payment;
mod payment_refs;
mod payment_vec;
mod token_id;

pub use rewa_or_dcdt_token_identifier::RewaOrDcdtTokenIdentifier;
pub use rewa_or_dcdt_token_payment::RewaOrDcdtTokenPayment;
pub use rewa_or_dcdt_token_payment_refs::RewaOrDcdtTokenPaymentRefs;
pub use rewa_or_multi_dcdt_payment::{RewaOrMultiDcdtPayment, RewaOrMultiDcdtPaymentRefs};
pub use dcdt_token_data::DcdtTokenData;
pub use dcdt_token_identifier::{DcdtTokenIdentifier, TokenIdentifier};
pub use dcdt_token_payment::{DcdtTokenPayment, DcdtTokenPaymentRefs, MultiDcdtPayment};
pub use multi_rewa_or_dcdt_token_payment::MultiRewaOrDcdtPayment;
pub use multi_transfer_marker::{MultiTransfer, MultiTransferMarkerArg};
pub use payment::Payment;
pub use payment_refs::PaymentRefs;
pub use payment_vec::PaymentVec;
pub use token_id::TokenId;

/// The old representation of the REWA token, before REWA-000000.
pub(crate) const LEGACY_REWA_REPRESENTATION: &[u8; 4] = b"REWA";
