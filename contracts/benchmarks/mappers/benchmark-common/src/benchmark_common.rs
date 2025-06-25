#![no_std]

numbat_wasm::imports!();

#[numbat_wasm::module]
pub trait BenchmarkCommon {
    fn append_index(&self, base: &ManagedBuffer, index: usize) -> ManagedBuffer {
        let mut concatenated = base.clone();
        concatenated.append_u32_be(index as u32);
        concatenated
    }
}
