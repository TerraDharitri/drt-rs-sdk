#![no_std]

mod dns_proxy;

numbat_wasm::imports!();

/// The module deals with registering usernames in a DNS contract.
#[numbat_wasm_derive::module]
pub trait DnsModule {
	#[proxy]
	fn dns_proxy(&self, to: Address) -> dns_proxy::Proxy<Self::SendApi>;

	#[payable("REWA")]
	#[endpoint(dnsRegister)]
	fn dns_register(
		&self,
		dns_address: Address,
		name: BoxedBytes,
		#[payment] payment: Self::BigUint,
	) -> SCResult<AsyncCall<Self::SendApi>> {
		only_owner!(self, "only owner can call dnsRegister");

		Ok(self
			.dns_proxy(dns_address)
			.register(name, payment)
			.async_call())
	}
}
