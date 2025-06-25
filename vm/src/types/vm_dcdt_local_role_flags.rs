#![allow(clippy::bad_bit_mask)]

use super::DcdtLocalRole;
use bitflags::bitflags;

bitflags! {
    /// The VM implementation for DcdtLocalRoleFlags, used internally in builtin functions.
    ///
    /// There is another near-identical implementation in the framework, used for communicating with the VM.
    ///
    /// It might be a good idea to move it to some "common ground" crate, between the framework and the VM.
    pub struct DcdtLocalRoleFlags: u64 {
        const NONE                  = 0b00000000;
        const MINT                  = 0b00000001;
        const BURN                  = 0b00000010;
        const NFT_CREATE            = 0b00000100;
        const NFT_ADD_QUANTITY      = 0b00001000;
        const NFT_BURN              = 0b00010000;
        const NFT_ADD_URI           = 0b00100000;
        const NFT_UPDATE_ATTRIBUTES = 0b01000000;
        const TRANSFER              = 0b10000000;
    }
}

impl DcdtLocalRoleFlags {
    pub fn has_role(&self, role: &DcdtLocalRole) -> bool {
        *self & role.to_flag() != DcdtLocalRoleFlags::NONE
    }

    pub fn iter_roles(&self) -> impl Iterator<Item = &DcdtLocalRole> {
        DcdtLocalRole::iter_all().filter(move |role| self.has_role(role))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_flags_has_role() {
        let flags = DcdtLocalRoleFlags::MINT;
        assert!(flags.has_role(&DcdtLocalRole::Mint));
        let flags = DcdtLocalRoleFlags::MINT | DcdtLocalRoleFlags::BURN;
        assert!(flags.has_role(&DcdtLocalRole::Mint));
        let flags = DcdtLocalRoleFlags::NONE;
        assert!(!flags.has_role(&DcdtLocalRole::Mint));
        let flags = DcdtLocalRoleFlags::BURN;
        assert!(!flags.has_role(&DcdtLocalRole::Mint));
    }

    #[test]
    fn test_flags_iter_role() {
        let flags = DcdtLocalRoleFlags::MINT;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&DcdtLocalRole>>(),
            alloc::vec![&DcdtLocalRole::Mint],
        );

        let flags = DcdtLocalRoleFlags::MINT | DcdtLocalRoleFlags::BURN;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&DcdtLocalRole>>(),
            alloc::vec![&DcdtLocalRole::Mint, &DcdtLocalRole::Burn],
        );

        let flags = DcdtLocalRoleFlags::NONE;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&DcdtLocalRole>>(),
            Vec::<&DcdtLocalRole>::new(),
        );
    }
}
