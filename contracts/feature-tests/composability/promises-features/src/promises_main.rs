#![no_std]
#![allow(clippy::type_complexity)]

mod call_promise_direct;
mod call_promises;

numbat_wasm::imports!();

/// Test contract for investigating the new async call framework.
#[numbat_wasm::contract]
pub trait PromisesFeatures:
    call_promises::CallPromisesModule + call_promise_direct::CallPromisesDirectModule
{
    #[init]
    fn init(&self) {}
}
