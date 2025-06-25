mod dns_proxy {
    numbat_wasm::imports!();

    #[numbat_wasm::proxy]
    pub trait Dns {
        #[payable("REWA")]
        #[endpoint]
        fn register(&self, name: &ManagedBuffer);
    }
}

numbat_wasm::imports!();

/// Standard smart contract module that deals with registering usernames in a DNS contract.
///
/// Numbat usernames/herotags need to be requested by the beneficiary.
/// For a contract, this means that they need an endpoint via which to request a username from the DNS.
///
#[numbat_wasm::module]
pub trait DnsModule {
    #[proxy]
    fn dns_proxy(&self, to: ManagedAddress) -> dns_proxy::Proxy<Self::Api>;

    #[payable("REWA")]
    #[only_owner]
    #[endpoint(dnsRegister)]
    fn dns_register(
        &self,
        dns_address: ManagedAddress,
        name: ManagedBuffer,
        #[payment] payment: BigUint,
    ) {
        self.dns_proxy(dns_address)
            .register(&name)
            .with_rewa_transfer(payment)
            .async_call()
            .call_and_exit()
    }
}
