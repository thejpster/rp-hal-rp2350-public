#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reset: RESET,
    wdsel: WDSEL,
    reset_done: RESET_DONE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn reset(&self) -> &RESET {
        &self.reset
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn wdsel(&self) -> &WDSEL {
        &self.wdsel
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn reset_done(&self) -> &RESET_DONE {
        &self.reset_done
    }
}
#[doc = "RESET (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@reset`]
module"]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = ""]
pub mod reset;
#[doc = "WDSEL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`wdsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@wdsel`]
module"]
pub type WDSEL = crate::Reg<wdsel::WDSEL_SPEC>;
#[doc = ""]
pub mod wdsel;
#[doc = "RESET_DONE (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`reset_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@reset_done`]
module"]
pub type RESET_DONE = crate::Reg<reset_done::RESET_DONE_SPEC>;
#[doc = ""]
pub mod reset_done;
