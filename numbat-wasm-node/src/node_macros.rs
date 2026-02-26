#[macro_export]
macro_rules! wasm_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)* ) ) => {
        pub use numbat_wasm_output;

        #[no_mangle]
        fn init() {
            $mod_name::endpoints::init::<numbat_wasm_node::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<numbat_wasm_node::VmApiImpl>();
            }
        )*
    };
}

#[macro_export]
macro_rules! external_view_wasm_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)* ) ) => {
        pub use numbat_wasm_output;

        #[no_mangle]
        fn init() {
            numbat_wasm_node::numbat_wasm::external_view_contract::external_view_contract_constructor::<numbat_wasm_node::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<numbat_wasm_node::numbat_wasm::api::ExternalViewApi<numbat_wasm_node::VmApiImpl>>();
            }
        )*
    };
}

#[macro_export]
macro_rules! wasm_empty_callback {
    () => {
        #[allow(non_snake_case)]
        #[no_mangle]
        fn callBack() {}
    };
}
