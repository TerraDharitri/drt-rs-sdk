use crate::types::{AnnotatedValue, BigUint, ManagedRef};

use super::TxEnv;

pub trait TxMoaValue<Env>: AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
}

impl<Env> TxMoaValue<Env> for BigUint<Env::Api> where Env: TxEnv {}
impl<Env> TxMoaValue<Env> for &BigUint<Env::Api> where Env: TxEnv {}
impl<Env> TxMoaValue<Env> for ManagedRef<'_, Env::Api, BigUint<Env::Api>> where Env: TxEnv {}
impl<Env> TxMoaValue<Env> for u64 where Env: TxEnv {}
