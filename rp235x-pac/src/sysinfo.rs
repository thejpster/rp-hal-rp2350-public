#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    chip_id: CHIP_ID,
    package_sel: PACKAGE_SEL,
    platform: PLATFORM,
    _reserved3: [u8; 0x08],
    gitref_rp2350: GITREF_RP2350,
}
impl RegisterBlock {
    #[doc = "0x00 - JEDEC JEP-106 compliant chip identifier."]
    #[inline(always)]
    pub const fn chip_id(&self) -> &CHIP_ID {
        &self.chip_id
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn package_sel(&self) -> &PACKAGE_SEL {
        &self.package_sel
    }
    #[doc = "0x08 - Platform register. Allows software to know what environment it is running in during pre-production development. Post-production, the PLATFORM is always ASIC, non-SIM."]
    #[inline(always)]
    pub const fn platform(&self) -> &PLATFORM {
        &self.platform
    }
    #[doc = "0x14 - Git hash of the chip source. Used to identify chip version."]
    #[inline(always)]
    pub const fn gitref_rp2350(&self) -> &GITREF_RP2350 {
        &self.gitref_rp2350
    }
}
#[doc = "CHIP_ID (rw) register accessor: JEDEC JEP-106 compliant chip identifier.  

You can [`read`](crate::Reg::read) this register and get [`chip_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chip_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chip_id`]
module"]
pub type CHIP_ID = crate::Reg<chip_id::CHIP_ID_SPEC>;
#[doc = "JEDEC JEP-106 compliant chip identifier."]
pub mod chip_id;
#[doc = "PACKAGE_SEL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`package_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`package_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@package_sel`]
module"]
pub type PACKAGE_SEL = crate::Reg<package_sel::PACKAGE_SEL_SPEC>;
#[doc = ""]
pub mod package_sel;
#[doc = "PLATFORM (rw) register accessor: Platform register. Allows software to know what environment it is running in during pre-production development. Post-production, the PLATFORM is always ASIC, non-SIM.  

You can [`read`](crate::Reg::read) this register and get [`platform::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`platform::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@platform`]
module"]
pub type PLATFORM = crate::Reg<platform::PLATFORM_SPEC>;
#[doc = "Platform register. Allows software to know what environment it is running in during pre-production development. Post-production, the PLATFORM is always ASIC, non-SIM."]
pub mod platform;
#[doc = "GITREF_RP2350 (rw) register accessor: Git hash of the chip source. Used to identify chip version.  

You can [`read`](crate::Reg::read) this register and get [`gitref_rp2350::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gitref_rp2350::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gitref_rp2350`]
module"]
pub type GITREF_RP2350 = crate::Reg<gitref_rp2350::GITREF_RP2350_SPEC>;
#[doc = "Git hash of the chip source. Used to identify chip version."]
pub mod gitref_rp2350;
