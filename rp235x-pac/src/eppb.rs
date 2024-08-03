#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    nmi_mask0: NMI_MASK0,
    nmi_mask1: NMI_MASK1,
    sleepctrl: SLEEPCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - NMI mask for IRQs 0 through 31. This register is core-local, and is reset by a processor warm reset."]
    #[inline(always)]
    pub const fn nmi_mask0(&self) -> &NMI_MASK0 {
        &self.nmi_mask0
    }
    #[doc = "0x04 - NMI mask for IRQs 0 though 51. This register is core-local, and is reset by a processor warm reset."]
    #[inline(always)]
    pub const fn nmi_mask1(&self) -> &NMI_MASK1 {
        &self.nmi_mask1
    }
    #[doc = "0x08 - Nonstandard sleep control register"]
    #[inline(always)]
    pub const fn sleepctrl(&self) -> &SLEEPCTRL {
        &self.sleepctrl
    }
}
#[doc = "NMI_MASK0 (rw) register accessor: NMI mask for IRQs 0 through 31. This register is core-local, and is reset by a processor warm reset.  

You can [`read`](crate::Reg::read) this register and get [`nmi_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nmi_mask0`]
module"]
pub type NMI_MASK0 = crate::Reg<nmi_mask0::NMI_MASK0_SPEC>;
#[doc = "NMI mask for IRQs 0 through 31. This register is core-local, and is reset by a processor warm reset."]
pub mod nmi_mask0;
#[doc = "NMI_MASK1 (rw) register accessor: NMI mask for IRQs 0 though 51. This register is core-local, and is reset by a processor warm reset.  

You can [`read`](crate::Reg::read) this register and get [`nmi_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nmi_mask1`]
module"]
pub type NMI_MASK1 = crate::Reg<nmi_mask1::NMI_MASK1_SPEC>;
#[doc = "NMI mask for IRQs 0 though 51. This register is core-local, and is reset by a processor warm reset."]
pub mod nmi_mask1;
#[doc = "SLEEPCTRL (rw) register accessor: Nonstandard sleep control register  

You can [`read`](crate::Reg::read) this register and get [`sleepctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleepctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sleepctrl`]
module"]
pub type SLEEPCTRL = crate::Reg<sleepctrl::SLEEPCTRL_SPEC>;
#[doc = "Nonstandard sleep control register"]
pub mod sleepctrl;
