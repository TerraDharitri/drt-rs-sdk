pub struct DeprecatedVMHook {
    pub name: &'static str,
    pub note: &'static str,
}

impl DeprecatedVMHook {
    pub const fn new(name: &'static str, note: &'static str) -> Self {
        Self { name, note }
    }
}

pub const DEPRECATED_VM_HOOKS_1_5: &[DeprecatedVMHook] = &[
    DeprecatedVMHook::new(
        "getArgument",
        "Arguments are now processed via `mBufferGetArgument`",
    ),
    DeprecatedVMHook::new(
        "getCallValue",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getDCDTValue",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getDCDTValueByIndex",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getDCDTTokenName",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getDCDTTokenNameByIndex",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getDCDTTokenNonce",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getDCDTTokenNonceByIndex",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getDCDTTokenType",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getDCDTTokenTypeByIndex",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getNumDCDTTransfers",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getCallValueTokenName",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getCallValueTokenNameByIndex",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "writeEventLog",
        "Events are now logged via `managedWriteLog`",
    ),
];

pub(super) fn deprecated_vm_hooks_1_5(name: &str) -> Option<&'static DeprecatedVMHook> {
    DEPRECATED_VM_HOOKS_1_5
        .iter()
        .find(|hook| hook.name == name)
}
