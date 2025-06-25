#[macro_export]
macro_rules! wasm_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)+ ) ) => { 
        pub use numbat_wasm_output;

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name(numbat_wasm_node::vm_api());
            }
        )+
    };
}
