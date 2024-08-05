#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    direct_csr: DIRECT_CSR,
    direct_tx: DIRECT_TX,
    direct_rx: DIRECT_RX,
    m0_timing: M0_TIMING,
    m0_rfmt: M0_RFMT,
    m0_rcmd: M0_RCMD,
    m0_wfmt: M0_WFMT,
    m0_wcmd: M0_WCMD,
    m1_timing: M1_TIMING,
    m1_rfmt: M1_RFMT,
    m1_rcmd: M1_RCMD,
    m1_wfmt: M1_WFMT,
    m1_wcmd: M1_WCMD,
    atrans0: ATRANS0,
    atrans1: ATRANS1,
    atrans2: ATRANS2,
    atrans3: ATRANS3,
    atrans4: ATRANS4,
    atrans5: ATRANS5,
    atrans6: ATRANS6,
    atrans7: ATRANS7,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and status for direct serial mode Direct serial mode allows the processor to send and receive raw serial frames, for programming, configuration and control of the external memory devices. Only SPI mode 0 (CPOL=0 CPHA=0) is supported."]
    #[inline(always)]
    pub const fn direct_csr(&self) -> &DIRECT_CSR {
        &self.direct_csr
    }
    #[doc = "0x04 - Transmit FIFO for direct mode"]
    #[inline(always)]
    pub const fn direct_tx(&self) -> &DIRECT_TX {
        &self.direct_tx
    }
    #[doc = "0x08 - Receive FIFO for direct mode"]
    #[inline(always)]
    pub const fn direct_rx(&self) -> &DIRECT_RX {
        &self.direct_rx
    }
    #[doc = "0x0c - Timing configuration register for memory address window 0."]
    #[inline(always)]
    pub const fn m0_timing(&self) -> &M0_TIMING {
        &self.m0_timing
    }
    #[doc = "0x10 - Read transfer format configuration for memory address window 0. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M0_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration."]
    #[inline(always)]
    pub const fn m0_rfmt(&self) -> &M0_RFMT {
        &self.m0_rfmt
    }
    #[doc = "0x14 - Command constants used for reads from memory address window 0. The reset value of the M0_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration."]
    #[inline(always)]
    pub const fn m0_rcmd(&self) -> &M0_RCMD {
        &self.m0_rcmd
    }
    #[doc = "0x18 - Write transfer format configuration for memory address window 0. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M0_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M0 bit, as XIP memory is read-only by default."]
    #[inline(always)]
    pub const fn m0_wfmt(&self) -> &M0_WFMT {
        &self.m0_wfmt
    }
    #[doc = "0x1c - Command constants used for writes to memory address window 0. The reset value of the M0_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration."]
    #[inline(always)]
    pub const fn m0_wcmd(&self) -> &M0_WCMD {
        &self.m0_wcmd
    }
    #[doc = "0x20 - Timing configuration register for memory address window 1."]
    #[inline(always)]
    pub const fn m1_timing(&self) -> &M1_TIMING {
        &self.m1_timing
    }
    #[doc = "0x24 - Read transfer format configuration for memory address window 1. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M1_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration."]
    #[inline(always)]
    pub const fn m1_rfmt(&self) -> &M1_RFMT {
        &self.m1_rfmt
    }
    #[doc = "0x28 - Command constants used for reads from memory address window 1. The reset value of the M1_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration."]
    #[inline(always)]
    pub const fn m1_rcmd(&self) -> &M1_RCMD {
        &self.m1_rcmd
    }
    #[doc = "0x2c - Write transfer format configuration for memory address window 1. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M1_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M1 bit, as XIP memory is read-only by default."]
    #[inline(always)]
    pub const fn m1_wfmt(&self) -> &M1_WFMT {
        &self.m1_wfmt
    }
    #[doc = "0x30 - Command constants used for writes to memory address window 1. The reset value of the M1_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration."]
    #[inline(always)]
    pub const fn m1_wcmd(&self) -> &M1_WCMD {
        &self.m1_wcmd
    }
    #[doc = "0x34 - Configure address translation for XIP virtual addresses 0x000000 through 0x3fffff (a 4 MiB window starting at +0 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
    #[inline(always)]
    pub const fn atrans0(&self) -> &ATRANS0 {
        &self.atrans0
    }
    #[doc = "0x38 - Configure address translation for XIP virtual addresses 0x400000 through 0x7fffff (a 4 MiB window starting at +4 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
    #[inline(always)]
    pub const fn atrans1(&self) -> &ATRANS1 {
        &self.atrans1
    }
    #[doc = "0x3c - Configure address translation for XIP virtual addresses 0x800000 through 0xbfffff (a 4 MiB window starting at +8 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
    #[inline(always)]
    pub const fn atrans2(&self) -> &ATRANS2 {
        &self.atrans2
    }
    #[doc = "0x40 - Configure address translation for XIP virtual addresses 0xc00000 through 0xffffff (a 4 MiB window starting at +12 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
    #[inline(always)]
    pub const fn atrans3(&self) -> &ATRANS3 {
        &self.atrans3
    }
    #[doc = "0x44 - Configure address translation for XIP virtual addresses 0x1000000 through 0x13fffff (a 4 MiB window starting at +16 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
    #[inline(always)]
    pub const fn atrans4(&self) -> &ATRANS4 {
        &self.atrans4
    }
    #[doc = "0x48 - Configure address translation for XIP virtual addresses 0x1400000 through 0x17fffff (a 4 MiB window starting at +20 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
    #[inline(always)]
    pub const fn atrans5(&self) -> &ATRANS5 {
        &self.atrans5
    }
    #[doc = "0x4c - Configure address translation for XIP virtual addresses 0x1800000 through 0x1bfffff (a 4 MiB window starting at +24 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
    #[inline(always)]
    pub const fn atrans6(&self) -> &ATRANS6 {
        &self.atrans6
    }
    #[doc = "0x50 - Configure address translation for XIP virtual addresses 0x1c00000 through 0x1ffffff (a 4 MiB window starting at +28 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
    #[inline(always)]
    pub const fn atrans7(&self) -> &ATRANS7 {
        &self.atrans7
    }
}
#[doc = "DIRECT_CSR (rw) register accessor: Control and status for direct serial mode Direct serial mode allows the processor to send and receive raw serial frames, for programming, configuration and control of the external memory devices. Only SPI mode 0 (CPOL=0 CPHA=0) is supported.  

You can [`read`](crate::Reg::read) this register and get [`direct_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direct_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@direct_csr`]
module"]
pub type DIRECT_CSR = crate::Reg<direct_csr::DIRECT_CSR_SPEC>;
#[doc = "Control and status for direct serial mode Direct serial mode allows the processor to send and receive raw serial frames, for programming, configuration and control of the external memory devices. Only SPI mode 0 (CPOL=0 CPHA=0) is supported."]
pub mod direct_csr;
#[doc = "DIRECT_TX (rw) register accessor: Transmit FIFO for direct mode  

You can [`read`](crate::Reg::read) this register and get [`direct_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direct_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@direct_tx`]
module"]
pub type DIRECT_TX = crate::Reg<direct_tx::DIRECT_TX_SPEC>;
#[doc = "Transmit FIFO for direct mode"]
pub mod direct_tx;
#[doc = "DIRECT_RX (rw) register accessor: Receive FIFO for direct mode  

You can [`read`](crate::Reg::read) this register and get [`direct_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direct_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@direct_rx`]
module"]
pub type DIRECT_RX = crate::Reg<direct_rx::DIRECT_RX_SPEC>;
#[doc = "Receive FIFO for direct mode"]
pub mod direct_rx;
#[doc = "M0_TIMING (rw) register accessor: Timing configuration register for memory address window 0.  

You can [`read`](crate::Reg::read) this register and get [`m0_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@m0_timing`]
module"]
pub type M0_TIMING = crate::Reg<m0_timing::M0_TIMING_SPEC>;
#[doc = "Timing configuration register for memory address window 0."]
pub mod m0_timing;
#[doc = "M0_RFMT (rw) register accessor: Read transfer format configuration for memory address window 0. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M0_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration.  

You can [`read`](crate::Reg::read) this register and get [`m0_rfmt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_rfmt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@m0_rfmt`]
module"]
pub type M0_RFMT = crate::Reg<m0_rfmt::M0_RFMT_SPEC>;
#[doc = "Read transfer format configuration for memory address window 0. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M0_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration."]
pub mod m0_rfmt;
#[doc = "M0_RCMD (rw) register accessor: Command constants used for reads from memory address window 0. The reset value of the M0_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration.  

You can [`read`](crate::Reg::read) this register and get [`m0_rcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_rcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@m0_rcmd`]
module"]
pub type M0_RCMD = crate::Reg<m0_rcmd::M0_RCMD_SPEC>;
#[doc = "Command constants used for reads from memory address window 0. The reset value of the M0_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration."]
pub mod m0_rcmd;
#[doc = "M0_WFMT (rw) register accessor: Write transfer format configuration for memory address window 0. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M0_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M0 bit, as XIP memory is read-only by default.  

You can [`read`](crate::Reg::read) this register and get [`m0_wfmt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_wfmt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@m0_wfmt`]
module"]
pub type M0_WFMT = crate::Reg<m0_wfmt::M0_WFMT_SPEC>;
#[doc = "Write transfer format configuration for memory address window 0. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M0_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M0 bit, as XIP memory is read-only by default."]
pub mod m0_wfmt;
#[doc = "M0_WCMD (rw) register accessor: Command constants used for writes to memory address window 0. The reset value of the M0_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration.  

You can [`read`](crate::Reg::read) this register and get [`m0_wcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_wcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@m0_wcmd`]
module"]
pub type M0_WCMD = crate::Reg<m0_wcmd::M0_WCMD_SPEC>;
#[doc = "Command constants used for writes to memory address window 0. The reset value of the M0_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration."]
pub mod m0_wcmd;
#[doc = "M1_TIMING (rw) register accessor: Timing configuration register for memory address window 1.  

You can [`read`](crate::Reg::read) this register and get [`m1_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@m1_timing`]
module"]
pub type M1_TIMING = crate::Reg<m1_timing::M1_TIMING_SPEC>;
#[doc = "Timing configuration register for memory address window 1."]
pub mod m1_timing;
#[doc = "M1_RFMT (rw) register accessor: Read transfer format configuration for memory address window 1. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M1_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration.  

You can [`read`](crate::Reg::read) this register and get [`m1_rfmt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_rfmt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@m1_rfmt`]
module"]
pub type M1_RFMT = crate::Reg<m1_rfmt::M1_RFMT_SPEC>;
#[doc = "Read transfer format configuration for memory address window 1. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M1_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration."]
pub mod m1_rfmt;
#[doc = "M1_RCMD (rw) register accessor: Command constants used for reads from memory address window 1. The reset value of the M1_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration.  

You can [`read`](crate::Reg::read) this register and get [`m1_rcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_rcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@m1_rcmd`]
module"]
pub type M1_RCMD = crate::Reg<m1_rcmd::M1_RCMD_SPEC>;
#[doc = "Command constants used for reads from memory address window 1. The reset value of the M1_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration."]
pub mod m1_rcmd;
#[doc = "M1_WFMT (rw) register accessor: Write transfer format configuration for memory address window 1. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M1_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M1 bit, as XIP memory is read-only by default.  

You can [`read`](crate::Reg::read) this register and get [`m1_wfmt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_wfmt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@m1_wfmt`]
module"]
pub type M1_WFMT = crate::Reg<m1_wfmt::M1_WFMT_SPEC>;
#[doc = "Write transfer format configuration for memory address window 1. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M1_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M1 bit, as XIP memory is read-only by default."]
pub mod m1_wfmt;
#[doc = "M1_WCMD (rw) register accessor: Command constants used for writes to memory address window 1. The reset value of the M1_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration.  

You can [`read`](crate::Reg::read) this register and get [`m1_wcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_wcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@m1_wcmd`]
module"]
pub type M1_WCMD = crate::Reg<m1_wcmd::M1_WCMD_SPEC>;
#[doc = "Command constants used for writes to memory address window 1. The reset value of the M1_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration."]
pub mod m1_wcmd;
#[doc = "ATRANS0 (rw) register accessor: Configure address translation for XIP virtual addresses 0x000000 through 0x3fffff (a 4 MiB window starting at +0 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.  

You can [`read`](crate::Reg::read) this register and get [`atrans0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atrans0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@atrans0`]
module"]
pub type ATRANS0 = crate::Reg<atrans0::ATRANS0_SPEC>;
#[doc = "Configure address translation for XIP virtual addresses 0x000000 through 0x3fffff (a 4 MiB window starting at +0 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
pub mod atrans0;
#[doc = "ATRANS1 (rw) register accessor: Configure address translation for XIP virtual addresses 0x400000 through 0x7fffff (a 4 MiB window starting at +4 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.  

You can [`read`](crate::Reg::read) this register and get [`atrans1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atrans1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@atrans1`]
module"]
pub type ATRANS1 = crate::Reg<atrans1::ATRANS1_SPEC>;
#[doc = "Configure address translation for XIP virtual addresses 0x400000 through 0x7fffff (a 4 MiB window starting at +4 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
pub mod atrans1;
#[doc = "ATRANS2 (rw) register accessor: Configure address translation for XIP virtual addresses 0x800000 through 0xbfffff (a 4 MiB window starting at +8 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.  

You can [`read`](crate::Reg::read) this register and get [`atrans2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atrans2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@atrans2`]
module"]
pub type ATRANS2 = crate::Reg<atrans2::ATRANS2_SPEC>;
#[doc = "Configure address translation for XIP virtual addresses 0x800000 through 0xbfffff (a 4 MiB window starting at +8 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
pub mod atrans2;
#[doc = "ATRANS3 (rw) register accessor: Configure address translation for XIP virtual addresses 0xc00000 through 0xffffff (a 4 MiB window starting at +12 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.  

You can [`read`](crate::Reg::read) this register and get [`atrans3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atrans3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@atrans3`]
module"]
pub type ATRANS3 = crate::Reg<atrans3::ATRANS3_SPEC>;
#[doc = "Configure address translation for XIP virtual addresses 0xc00000 through 0xffffff (a 4 MiB window starting at +12 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
pub mod atrans3;
#[doc = "ATRANS4 (rw) register accessor: Configure address translation for XIP virtual addresses 0x1000000 through 0x13fffff (a 4 MiB window starting at +16 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.  

You can [`read`](crate::Reg::read) this register and get [`atrans4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atrans4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@atrans4`]
module"]
pub type ATRANS4 = crate::Reg<atrans4::ATRANS4_SPEC>;
#[doc = "Configure address translation for XIP virtual addresses 0x1000000 through 0x13fffff (a 4 MiB window starting at +16 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
pub mod atrans4;
#[doc = "ATRANS5 (rw) register accessor: Configure address translation for XIP virtual addresses 0x1400000 through 0x17fffff (a 4 MiB window starting at +20 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.  

You can [`read`](crate::Reg::read) this register and get [`atrans5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atrans5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@atrans5`]
module"]
pub type ATRANS5 = crate::Reg<atrans5::ATRANS5_SPEC>;
#[doc = "Configure address translation for XIP virtual addresses 0x1400000 through 0x17fffff (a 4 MiB window starting at +20 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
pub mod atrans5;
#[doc = "ATRANS6 (rw) register accessor: Configure address translation for XIP virtual addresses 0x1800000 through 0x1bfffff (a 4 MiB window starting at +24 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.  

You can [`read`](crate::Reg::read) this register and get [`atrans6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atrans6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@atrans6`]
module"]
pub type ATRANS6 = crate::Reg<atrans6::ATRANS6_SPEC>;
#[doc = "Configure address translation for XIP virtual addresses 0x1800000 through 0x1bfffff (a 4 MiB window starting at +24 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
pub mod atrans6;
#[doc = "ATRANS7 (rw) register accessor: Configure address translation for XIP virtual addresses 0x1c00000 through 0x1ffffff (a 4 MiB window starting at +28 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.  

You can [`read`](crate::Reg::read) this register and get [`atrans7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atrans7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@atrans7`]
module"]
pub type ATRANS7 = crate::Reg<atrans7::ATRANS7_SPEC>;
#[doc = "Configure address translation for XIP virtual addresses 0x1c00000 through 0x1ffffff (a 4 MiB window starting at +28 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
pub mod atrans7;
