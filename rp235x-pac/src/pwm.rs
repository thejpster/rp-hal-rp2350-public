#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch: [CH; 12],
    en: EN,
    intr: INTR,
    irq0_inte: IRQ0_INTE,
    irq0_intf: IRQ0_INTF,
    irq0_ints: IRQ0_INTS,
    irq1_inte: IRQ1_INTE,
    irq1_intf: IRQ1_INTF,
    irq1_ints: IRQ1_INTS,
}
impl RegisterBlock {
    #[doc = "0x00..0xf0 - Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0xf0 - Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0xf0 - This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
    #[inline(always)]
    pub const fn en(&self) -> &EN {
        &self.en
    }
    #[doc = "0xf4 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0xf8 - Interrupt Enable for irq0"]
    #[inline(always)]
    pub const fn irq0_inte(&self) -> &IRQ0_INTE {
        &self.irq0_inte
    }
    #[doc = "0xfc - Interrupt Force for irq0"]
    #[inline(always)]
    pub const fn irq0_intf(&self) -> &IRQ0_INTF {
        &self.irq0_intf
    }
    #[doc = "0x100 - Interrupt status after masking &amp; forcing for irq0"]
    #[inline(always)]
    pub const fn irq0_ints(&self) -> &IRQ0_INTS {
        &self.irq0_ints
    }
    #[doc = "0x104 - Interrupt Enable for irq1"]
    #[inline(always)]
    pub const fn irq1_inte(&self) -> &IRQ1_INTE {
        &self.irq1_inte
    }
    #[doc = "0x108 - Interrupt Force for irq1"]
    #[inline(always)]
    pub const fn irq1_intf(&self) -> &IRQ1_INTF {
        &self.irq1_intf
    }
    #[doc = "0x10c - Interrupt status after masking &amp; forcing for irq1"]
    #[inline(always)]
    pub const fn irq1_ints(&self) -> &IRQ1_INTS {
        &self.irq1_ints
    }
}
#[doc = "Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH*_CC, CH*_CSR, CH*_CTR, CH*_DIV, CH*_TOP"]
pub mod ch;
#[doc = "EN (rw) register accessor: This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR.  

You can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@en`]
module"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
pub mod en;
#[doc = "INTR (rw) register accessor: Raw Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "IRQ0_INTE (rw) register accessor: Interrupt Enable for irq0  

You can [`read`](crate::Reg::read) this register and get [`irq0_inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq0_inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq0_inte`]
module"]
pub type IRQ0_INTE = crate::Reg<irq0_inte::IRQ0_INTE_SPEC>;
#[doc = "Interrupt Enable for irq0"]
pub mod irq0_inte;
#[doc = "IRQ0_INTF (rw) register accessor: Interrupt Force for irq0  

You can [`read`](crate::Reg::read) this register and get [`irq0_intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq0_intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq0_intf`]
module"]
pub type IRQ0_INTF = crate::Reg<irq0_intf::IRQ0_INTF_SPEC>;
#[doc = "Interrupt Force for irq0"]
pub mod irq0_intf;
#[doc = "IRQ0_INTS (rw) register accessor: Interrupt status after masking &amp; forcing for irq0  

You can [`read`](crate::Reg::read) this register and get [`irq0_ints::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq0_ints::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq0_ints`]
module"]
pub type IRQ0_INTS = crate::Reg<irq0_ints::IRQ0_INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing for irq0"]
pub mod irq0_ints;
#[doc = "IRQ1_INTE (rw) register accessor: Interrupt Enable for irq1  

You can [`read`](crate::Reg::read) this register and get [`irq1_inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq1_inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq1_inte`]
module"]
pub type IRQ1_INTE = crate::Reg<irq1_inte::IRQ1_INTE_SPEC>;
#[doc = "Interrupt Enable for irq1"]
pub mod irq1_inte;
#[doc = "IRQ1_INTF (rw) register accessor: Interrupt Force for irq1  

You can [`read`](crate::Reg::read) this register and get [`irq1_intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq1_intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq1_intf`]
module"]
pub type IRQ1_INTF = crate::Reg<irq1_intf::IRQ1_INTF_SPEC>;
#[doc = "Interrupt Force for irq1"]
pub mod irq1_intf;
#[doc = "IRQ1_INTS (rw) register accessor: Interrupt status after masking &amp; forcing for irq1  

You can [`read`](crate::Reg::read) this register and get [`irq1_ints::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq1_ints::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irq1_ints`]
module"]
pub type IRQ1_INTS = crate::Reg<irq1_ints::IRQ1_INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing for irq1"]
pub mod irq1_ints;
