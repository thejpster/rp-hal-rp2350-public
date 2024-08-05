#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    usbphy_dp_status: USBPHY_DP_STATUS,
    usbphy_dp_ctrl: USBPHY_DP_CTRL,
    usbphy_dm_status: USBPHY_DM_STATUS,
    usbphy_dm_ctrl: USBPHY_DM_CTRL,
    gpio_qspi: [GPIO_QSPI; 6],
    _reserved5: [u8; 0x01c0],
    irqsummary_proc0_secure: IRQSUMMARY_PROC0_SECURE,
    irqsummary_proc0_nonsecure: IRQSUMMARY_PROC0_NONSECURE,
    irqsummary_proc1_secure: IRQSUMMARY_PROC1_SECURE,
    irqsummary_proc1_nonsecure: IRQSUMMARY_PROC1_NONSECURE,
    irqsummary_dormant_wake_secure: IRQSUMMARY_DORMANT_WAKE_SECURE,
    irqsummary_dormant_wake_nonsecure: IRQSUMMARY_DORMANT_WAKE_NONSECURE,
    intr: INTR,
    proc0_inte: PROC0_INTE,
    proc0_intf: PROC0_INTF,
    proc0_ints: PROC0_INTS,
    proc1_inte: PROC1_INTE,
    proc1_intf: PROC1_INTF,
    proc1_ints: PROC1_INTS,
    dormant_wake_inte: DORMANT_WAKE_INTE,
    dormant_wake_intf: DORMANT_WAKE_INTF,
    dormant_wake_ints: DORMANT_WAKE_INTS,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn usbphy_dp_status(&self) -> &USBPHY_DP_STATUS {
        &self.usbphy_dp_status
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn usbphy_dp_ctrl(&self) -> &USBPHY_DP_CTRL {
        &self.usbphy_dp_ctrl
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn usbphy_dm_status(&self) -> &USBPHY_DM_STATUS {
        &self.usbphy_dm_status
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn usbphy_dm_ctrl(&self) -> &USBPHY_DM_CTRL {
        &self.usbphy_dm_ctrl
    }
    #[doc = "0x10..0x40 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub const fn gpio_qspi(&self, n: usize) -> &GPIO_QSPI {
        &self.gpio_qspi[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x40 - Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub fn gpio_qspi_iter(&self) -> impl Iterator<Item = &GPIO_QSPI> {
        self.gpio_qspi.iter()
    }
    #[doc = "0x10..0x18 - Cluster GPIO_QSPISCLK, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub const fn gpio_qspisclk(&self) -> &GPIO_QSPI {
        self.gpio_qspi(0)
    }
    #[doc = "0x18..0x20 - Cluster GPIO_QSPISS, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub const fn gpio_qspiss(&self) -> &GPIO_QSPI {
        self.gpio_qspi(1)
    }
    #[doc = "0x20..0x28 - Cluster GPIO_QSPISD0, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub const fn gpio_qspisd0(&self) -> &GPIO_QSPI {
        self.gpio_qspi(2)
    }
    #[doc = "0x28..0x30 - Cluster GPIO_QSPISD1, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub const fn gpio_qspisd1(&self) -> &GPIO_QSPI {
        self.gpio_qspi(3)
    }
    #[doc = "0x30..0x38 - Cluster GPIO_QSPISD2, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub const fn gpio_qspisd2(&self) -> &GPIO_QSPI {
        self.gpio_qspi(4)
    }
    #[doc = "0x38..0x40 - Cluster GPIO_QSPISD3, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
    #[inline(always)]
    pub const fn gpio_qspisd3(&self) -> &GPIO_QSPI {
        self.gpio_qspi(5)
    }
    #[doc = "0x200 - "]
    #[inline(always)]
    pub const fn irqsummary_proc0_secure(&self) -> &IRQSUMMARY_PROC0_SECURE {
        &self.irqsummary_proc0_secure
    }
    #[doc = "0x204 - "]
    #[inline(always)]
    pub const fn irqsummary_proc0_nonsecure(&self) -> &IRQSUMMARY_PROC0_NONSECURE {
        &self.irqsummary_proc0_nonsecure
    }
    #[doc = "0x208 - "]
    #[inline(always)]
    pub const fn irqsummary_proc1_secure(&self) -> &IRQSUMMARY_PROC1_SECURE {
        &self.irqsummary_proc1_secure
    }
    #[doc = "0x20c - "]
    #[inline(always)]
    pub const fn irqsummary_proc1_nonsecure(&self) -> &IRQSUMMARY_PROC1_NONSECURE {
        &self.irqsummary_proc1_nonsecure
    }
    #[doc = "0x210 - "]
    #[inline(always)]
    pub const fn irqsummary_dormant_wake_secure(&self) -> &IRQSUMMARY_DORMANT_WAKE_SECURE {
        &self.irqsummary_dormant_wake_secure
    }
    #[doc = "0x214 - "]
    #[inline(always)]
    pub const fn irqsummary_dormant_wake_nonsecure(&self) -> &IRQSUMMARY_DORMANT_WAKE_NONSECURE {
        &self.irqsummary_dormant_wake_nonsecure
    }
    #[doc = "0x218 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x21c - Interrupt Enable for proc0"]
    #[inline(always)]
    pub const fn proc0_inte(&self) -> &PROC0_INTE {
        &self.proc0_inte
    }
    #[doc = "0x220 - Interrupt Force for proc0"]
    #[inline(always)]
    pub const fn proc0_intf(&self) -> &PROC0_INTF {
        &self.proc0_intf
    }
    #[doc = "0x224 - Interrupt status after masking &amp; forcing for proc0"]
    #[inline(always)]
    pub const fn proc0_ints(&self) -> &PROC0_INTS {
        &self.proc0_ints
    }
    #[doc = "0x228 - Interrupt Enable for proc1"]
    #[inline(always)]
    pub const fn proc1_inte(&self) -> &PROC1_INTE {
        &self.proc1_inte
    }
    #[doc = "0x22c - Interrupt Force for proc1"]
    #[inline(always)]
    pub const fn proc1_intf(&self) -> &PROC1_INTF {
        &self.proc1_intf
    }
    #[doc = "0x230 - Interrupt status after masking &amp; forcing for proc1"]
    #[inline(always)]
    pub const fn proc1_ints(&self) -> &PROC1_INTS {
        &self.proc1_ints
    }
    #[doc = "0x234 - Interrupt Enable for dormant_wake"]
    #[inline(always)]
    pub const fn dormant_wake_inte(&self) -> &DORMANT_WAKE_INTE {
        &self.dormant_wake_inte
    }
    #[doc = "0x238 - Interrupt Force for dormant_wake"]
    #[inline(always)]
    pub const fn dormant_wake_intf(&self) -> &DORMANT_WAKE_INTF {
        &self.dormant_wake_intf
    }
    #[doc = "0x23c - Interrupt status after masking &amp; forcing for dormant_wake"]
    #[inline(always)]
    pub const fn dormant_wake_ints(&self) -> &DORMANT_WAKE_INTS {
        &self.dormant_wake_ints
    }
}
#[doc = "USBPHY_DP_STATUS (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`usbphy_dp_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_dp_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usbphy_dp_status`]
module"]
pub type USBPHY_DP_STATUS = crate::Reg<usbphy_dp_status::USBPHY_DP_STATUS_SPEC>;
#[doc = ""]
pub mod usbphy_dp_status;
#[doc = "USBPHY_DP_CTRL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`usbphy_dp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_dp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usbphy_dp_ctrl`]
module"]
pub type USBPHY_DP_CTRL = crate::Reg<usbphy_dp_ctrl::USBPHY_DP_CTRL_SPEC>;
#[doc = ""]
pub mod usbphy_dp_ctrl;
#[doc = "USBPHY_DM_STATUS (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`usbphy_dm_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_dm_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usbphy_dm_status`]
module"]
pub type USBPHY_DM_STATUS = crate::Reg<usbphy_dm_status::USBPHY_DM_STATUS_SPEC>;
#[doc = ""]
pub mod usbphy_dm_status;
#[doc = "USBPHY_DM_CTRL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`usbphy_dm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_dm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usbphy_dm_ctrl`]
module"]
pub type USBPHY_DM_CTRL = crate::Reg<usbphy_dm_ctrl::USBPHY_DM_CTRL_SPEC>;
#[doc = ""]
pub mod usbphy_dm_ctrl;
#[doc = "Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
pub use self::gpio_qspi::GPIO_QSPI;
#[doc = r"Cluster"]
#[doc = "Cluster GPIO_QSPI%s, containing GPIO_QSPI_*_STATUS, GPIO_QSPI_*_CTRL"]
pub mod gpio_qspi;
#[doc = "IRQSUMMARY_PROC0_SECURE (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`irqsummary_proc0_secure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsummary_proc0_secure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irqsummary_proc0_secure`]
module"]
pub type IRQSUMMARY_PROC0_SECURE =
    crate::Reg<irqsummary_proc0_secure::IRQSUMMARY_PROC0_SECURE_SPEC>;
#[doc = ""]
pub mod irqsummary_proc0_secure;
#[doc = "IRQSUMMARY_PROC0_NONSECURE (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`irqsummary_proc0_nonsecure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsummary_proc0_nonsecure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irqsummary_proc0_nonsecure`]
module"]
pub type IRQSUMMARY_PROC0_NONSECURE =
    crate::Reg<irqsummary_proc0_nonsecure::IRQSUMMARY_PROC0_NONSECURE_SPEC>;
#[doc = ""]
pub mod irqsummary_proc0_nonsecure;
#[doc = "IRQSUMMARY_PROC1_SECURE (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`irqsummary_proc1_secure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsummary_proc1_secure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irqsummary_proc1_secure`]
module"]
pub type IRQSUMMARY_PROC1_SECURE =
    crate::Reg<irqsummary_proc1_secure::IRQSUMMARY_PROC1_SECURE_SPEC>;
#[doc = ""]
pub mod irqsummary_proc1_secure;
#[doc = "IRQSUMMARY_PROC1_NONSECURE (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`irqsummary_proc1_nonsecure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsummary_proc1_nonsecure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irqsummary_proc1_nonsecure`]
module"]
pub type IRQSUMMARY_PROC1_NONSECURE =
    crate::Reg<irqsummary_proc1_nonsecure::IRQSUMMARY_PROC1_NONSECURE_SPEC>;
#[doc = ""]
pub mod irqsummary_proc1_nonsecure;
#[doc = "IRQSUMMARY_DORMANT_WAKE_SECURE (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`irqsummary_dormant_wake_secure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsummary_dormant_wake_secure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irqsummary_dormant_wake_secure`]
module"]
pub type IRQSUMMARY_DORMANT_WAKE_SECURE =
    crate::Reg<irqsummary_dormant_wake_secure::IRQSUMMARY_DORMANT_WAKE_SECURE_SPEC>;
#[doc = ""]
pub mod irqsummary_dormant_wake_secure;
#[doc = "IRQSUMMARY_DORMANT_WAKE_NONSECURE (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`irqsummary_dormant_wake_nonsecure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsummary_dormant_wake_nonsecure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@irqsummary_dormant_wake_nonsecure`]
module"]
pub type IRQSUMMARY_DORMANT_WAKE_NONSECURE =
    crate::Reg<irqsummary_dormant_wake_nonsecure::IRQSUMMARY_DORMANT_WAKE_NONSECURE_SPEC>;
#[doc = ""]
pub mod irqsummary_dormant_wake_nonsecure;
#[doc = "INTR (rw) register accessor: Raw Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "PROC0_INTE (rw) register accessor: Interrupt Enable for proc0  

You can [`read`](crate::Reg::read) this register and get [`proc0_inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc0_inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc0_inte`]
module"]
pub type PROC0_INTE = crate::Reg<proc0_inte::PROC0_INTE_SPEC>;
#[doc = "Interrupt Enable for proc0"]
pub mod proc0_inte;
#[doc = "PROC0_INTF (rw) register accessor: Interrupt Force for proc0  

You can [`read`](crate::Reg::read) this register and get [`proc0_intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc0_intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc0_intf`]
module"]
pub type PROC0_INTF = crate::Reg<proc0_intf::PROC0_INTF_SPEC>;
#[doc = "Interrupt Force for proc0"]
pub mod proc0_intf;
#[doc = "PROC0_INTS (rw) register accessor: Interrupt status after masking &amp; forcing for proc0  

You can [`read`](crate::Reg::read) this register and get [`proc0_ints::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc0_ints::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc0_ints`]
module"]
pub type PROC0_INTS = crate::Reg<proc0_ints::PROC0_INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing for proc0"]
pub mod proc0_ints;
#[doc = "PROC1_INTE (rw) register accessor: Interrupt Enable for proc1  

You can [`read`](crate::Reg::read) this register and get [`proc1_inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc1_inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc1_inte`]
module"]
pub type PROC1_INTE = crate::Reg<proc1_inte::PROC1_INTE_SPEC>;
#[doc = "Interrupt Enable for proc1"]
pub mod proc1_inte;
#[doc = "PROC1_INTF (rw) register accessor: Interrupt Force for proc1  

You can [`read`](crate::Reg::read) this register and get [`proc1_intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc1_intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc1_intf`]
module"]
pub type PROC1_INTF = crate::Reg<proc1_intf::PROC1_INTF_SPEC>;
#[doc = "Interrupt Force for proc1"]
pub mod proc1_intf;
#[doc = "PROC1_INTS (rw) register accessor: Interrupt status after masking &amp; forcing for proc1  

You can [`read`](crate::Reg::read) this register and get [`proc1_ints::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc1_ints::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc1_ints`]
module"]
pub type PROC1_INTS = crate::Reg<proc1_ints::PROC1_INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing for proc1"]
pub mod proc1_ints;
#[doc = "DORMANT_WAKE_INTE (rw) register accessor: Interrupt Enable for dormant_wake  

You can [`read`](crate::Reg::read) this register and get [`dormant_wake_inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dormant_wake_inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dormant_wake_inte`]
module"]
pub type DORMANT_WAKE_INTE = crate::Reg<dormant_wake_inte::DORMANT_WAKE_INTE_SPEC>;
#[doc = "Interrupt Enable for dormant_wake"]
pub mod dormant_wake_inte;
#[doc = "DORMANT_WAKE_INTF (rw) register accessor: Interrupt Force for dormant_wake  

You can [`read`](crate::Reg::read) this register and get [`dormant_wake_intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dormant_wake_intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dormant_wake_intf`]
module"]
pub type DORMANT_WAKE_INTF = crate::Reg<dormant_wake_intf::DORMANT_WAKE_INTF_SPEC>;
#[doc = "Interrupt Force for dormant_wake"]
pub mod dormant_wake_intf;
#[doc = "DORMANT_WAKE_INTS (rw) register accessor: Interrupt status after masking &amp; forcing for dormant_wake  

You can [`read`](crate::Reg::read) this register and get [`dormant_wake_ints::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dormant_wake_ints::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dormant_wake_ints`]
module"]
pub type DORMANT_WAKE_INTS = crate::Reg<dormant_wake_ints::DORMANT_WAKE_INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing for dormant_wake"]
pub mod dormant_wake_ints;
