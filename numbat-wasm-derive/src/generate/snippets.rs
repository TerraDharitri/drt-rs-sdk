pub fn contract_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct ContractObj<A>
        where
            A: numbat_wasm::api::VMApi,
        {
            _phantom: core::marker::PhantomData<A>,
        }
    }
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
    quote! {
        impl<A> numbat_wasm::contract_base::ContractBase for ContractObj<A>
        where
            A: numbat_wasm::api::VMApi,
        {
            type Api = A;
        }
    }
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
    quote! {
        pub fn contract_obj<A>() -> ContractObj<A>
        where
            A: numbat_wasm::api::VMApi,
        {
            ContractObj {
                _phantom: core::marker::PhantomData,
            }
        }

        pub struct ContractBuilder;

        impl numbat_wasm::contract_base::CallableContractBuilder for self::ContractBuilder {
            fn new_contract_obj<A: numbat_wasm::api::VMApi>(
                &self,
            ) -> numbat_wasm::types::heap::Box<dyn numbat_wasm::contract_base::CallableContract> {
                numbat_wasm::types::heap::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
    }
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn impl_auto_impl() -> proc_macro2::TokenStream {
    quote! {
        impl<A> AutoImpl for ContractObj<A> where
            A: numbat_wasm::contract_base::ContractBase
                + numbat_wasm::api::ErrorApi
                + numbat_wasm::api::EndpointArgumentApi
                + numbat_wasm::api::EndpointFinishApi
                + numbat_wasm::api::ManagedTypeApi
        {
        }
    }
}

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
    quote! {
        impl<A> numbat_wasm::contract_base::CallableContract for ContractObj<A>
        where
            A: numbat_wasm::api::VMApi,
        {
            fn call(&self, fn_name: &str) -> bool {
                EndpointWrappers::call(self, fn_name)
            }
        }
    }
}

pub fn proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct Proxy<A>
        where
            A: numbat_wasm::api::VMApi + 'static,
        {
            pub address: numbat_wasm::types::ManagedOption<A, numbat_wasm::types::ManagedAddress<A>>,
        }

        impl<A> numbat_wasm::contract_base::ProxyObjBase for Proxy<A>
        where
            A: numbat_wasm::api::VMApi + 'static,
        {
            type Api = A;

            fn new_proxy_obj() -> Self {
                Proxy {
                    address: numbat_wasm::types::ManagedOption::none(),
                }
            }

            fn contract(mut self, address: numbat_wasm::types::ManagedAddress<Self::Api>) -> Self {
                self.address = numbat_wasm::types::ManagedOption::some(address);
                self
            }

            fn extract_opt_address(
                &mut self,
            ) -> numbat_wasm::types::ManagedOption<
                Self::Api,
                numbat_wasm::types::ManagedAddress<Self::Api>,
            > {
                core::mem::replace(&mut self.address, numbat_wasm::types::ManagedOption::none())
            }

            fn extract_address(&mut self) -> numbat_wasm::types::ManagedAddress<Self::Api> {
                self.extract_opt_address().unwrap_or_sc_panic(numbat_wasm::err_msg::RECIPIENT_ADDRESS_NOT_SET)
            }
        }
    }
}

pub fn callback_proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct CallbackProxyObj<A>
        where
            A: numbat_wasm::api::VMApi + 'static,
        {
            _phantom: core::marker::PhantomData<A>,
        }

        impl<A> numbat_wasm::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
        where
            A: numbat_wasm::api::VMApi + 'static,
        {
            type Api = A;

            fn new_cb_proxy_obj() -> Self {
                CallbackProxyObj {
                    _phantom: core::marker::PhantomData,
                }
            }
        }
    }
}

pub fn call_method_api_static_init() -> proc_macro2::TokenStream {
    quote! {
        <Self::Api as numbat_wasm::api::VMApi>::init_static();
    }
}
