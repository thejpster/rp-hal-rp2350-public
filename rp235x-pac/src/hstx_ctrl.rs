#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: CSR,
    bit0: BIT0,
    bit1: BIT1,
    bit2: BIT2,
    bit3: BIT3,
    bit4: BIT4,
    bit5: BIT5,
    bit6: BIT6,
    bit7: BIT7,
    expand_shift: EXPAND_SHIFT,
    expand_tmds: EXPAND_TMDS,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x04 - Data control register for output bit 0"]
    #[inline(always)]
    pub const fn bit0(&self) -> &BIT0 {
        &self.bit0
    }
    #[doc = "0x08 - Data control register for output bit 1"]
    #[inline(always)]
    pub const fn bit1(&self) -> &BIT1 {
        &self.bit1
    }
    #[doc = "0x0c - Data control register for output bit 2"]
    #[inline(always)]
    pub const fn bit2(&self) -> &BIT2 {
        &self.bit2
    }
    #[doc = "0x10 - Data control register for output bit 3"]
    #[inline(always)]
    pub const fn bit3(&self) -> &BIT3 {
        &self.bit3
    }
    #[doc = "0x14 - Data control register for output bit 4"]
    #[inline(always)]
    pub const fn bit4(&self) -> &BIT4 {
        &self.bit4
    }
    #[doc = "0x18 - Data control register for output bit 5"]
    #[inline(always)]
    pub const fn bit5(&self) -> &BIT5 {
        &self.bit5
    }
    #[doc = "0x1c - Data control register for output bit 6"]
    #[inline(always)]
    pub const fn bit6(&self) -> &BIT6 {
        &self.bit6
    }
    #[doc = "0x20 - Data control register for output bit 7"]
    #[inline(always)]
    pub const fn bit7(&self) -> &BIT7 {
        &self.bit7
    }
    #[doc = "0x24 - Configure the optional shifter inside the command expander"]
    #[inline(always)]
    pub const fn expand_shift(&self) -> &EXPAND_SHIFT {
        &self.expand_shift
    }
    #[doc = "0x28 - Configure the optional TMDS encoder inside the command expander"]
    #[inline(always)]
    pub const fn expand_tmds(&self) -> &EXPAND_TMDS {
        &self.expand_tmds
    }
}
#[doc = "CSR (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = ""]
pub mod csr;
#[doc = "BIT0 (rw) register accessor: Data control register for output bit 0  

You can [`read`](crate::Reg::read) this register and get [`bit0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bit0`]
module"]
pub type BIT0 = crate::Reg<bit0::BIT0_SPEC>;
#[doc = "Data control register for output bit 0"]
pub mod bit0;
#[doc = "BIT1 (rw) register accessor: Data control register for output bit 1  

You can [`read`](crate::Reg::read) this register and get [`bit1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bit1`]
module"]
pub type BIT1 = crate::Reg<bit1::BIT1_SPEC>;
#[doc = "Data control register for output bit 1"]
pub mod bit1;
#[doc = "BIT2 (rw) register accessor: Data control register for output bit 2  

You can [`read`](crate::Reg::read) this register and get [`bit2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bit2`]
module"]
pub type BIT2 = crate::Reg<bit2::BIT2_SPEC>;
#[doc = "Data control register for output bit 2"]
pub mod bit2;
#[doc = "BIT3 (rw) register accessor: Data control register for output bit 3  

You can [`read`](crate::Reg::read) this register and get [`bit3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bit3`]
module"]
pub type BIT3 = crate::Reg<bit3::BIT3_SPEC>;
#[doc = "Data control register for output bit 3"]
pub mod bit3;
#[doc = "BIT4 (rw) register accessor: Data control register for output bit 4  

You can [`read`](crate::Reg::read) this register and get [`bit4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bit4`]
module"]
pub type BIT4 = crate::Reg<bit4::BIT4_SPEC>;
#[doc = "Data control register for output bit 4"]
pub mod bit4;
#[doc = "BIT5 (rw) register accessor: Data control register for output bit 5  

You can [`read`](crate::Reg::read) this register and get [`bit5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bit5`]
module"]
pub type BIT5 = crate::Reg<bit5::BIT5_SPEC>;
#[doc = "Data control register for output bit 5"]
pub mod bit5;
#[doc = "BIT6 (rw) register accessor: Data control register for output bit 6  

You can [`read`](crate::Reg::read) this register and get [`bit6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bit6`]
module"]
pub type BIT6 = crate::Reg<bit6::BIT6_SPEC>;
#[doc = "Data control register for output bit 6"]
pub mod bit6;
#[doc = "BIT7 (rw) register accessor: Data control register for output bit 7  

You can [`read`](crate::Reg::read) this register and get [`bit7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bit7`]
module"]
pub type BIT7 = crate::Reg<bit7::BIT7_SPEC>;
#[doc = "Data control register for output bit 7"]
pub mod bit7;
#[doc = "EXPAND_SHIFT (rw) register accessor: Configure the optional shifter inside the command expander  

You can [`read`](crate::Reg::read) this register and get [`expand_shift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`expand_shift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@expand_shift`]
module"]
pub type EXPAND_SHIFT = crate::Reg<expand_shift::EXPAND_SHIFT_SPEC>;
#[doc = "Configure the optional shifter inside the command expander"]
pub mod expand_shift;
#[doc = "EXPAND_TMDS (rw) register accessor: Configure the optional TMDS encoder inside the command expander  

You can [`read`](crate::Reg::read) this register and get [`expand_tmds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`expand_tmds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@expand_tmds`]
module"]
pub type EXPAND_TMDS = crate::Reg<expand_tmds::EXPAND_TMDS_SPEC>;
#[doc = "Configure the optional TMDS encoder inside the command expander"]
pub mod expand_tmds;
