#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: CSR,
    wdata: WDATA,
    sum0: SUM0,
    sum1: SUM1,
    sum2: SUM2,
    sum3: SUM3,
    sum4: SUM4,
    sum5: SUM5,
    sum6: SUM6,
    sum7: SUM7,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x04 - Write data register  
 After pulsing START and writing 16 words of data to this register, WDATA_RDY will go low and the SHA-256 core will complete the digest of the current 512-bit block.  

 Software is responsible for ensuring the data is correctly padded and terminated to a whole number of 512-bit blocks.  

 After this, WDATA_RDY will return high, and more data can be written (if any).  

 This register supports word, halfword and byte writes, so that DMA from non-word-aligned buffers can be supported. The total amount of data per block remains the same (16 words, 32 halfwords or 64 bytes) and byte/halfword transfers must not be mixed within a block."]
    #[inline(always)]
    pub const fn wdata(&self) -> &WDATA {
        &self.wdata
    }
    #[doc = "0x08 - 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum0(&self) -> &SUM0 {
        &self.sum0
    }
    #[doc = "0x0c - 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum1(&self) -> &SUM1 {
        &self.sum1
    }
    #[doc = "0x10 - 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum2(&self) -> &SUM2 {
        &self.sum2
    }
    #[doc = "0x14 - 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum3(&self) -> &SUM3 {
        &self.sum3
    }
    #[doc = "0x18 - 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum4(&self) -> &SUM4 {
        &self.sum4
    }
    #[doc = "0x1c - 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum5(&self) -> &SUM5 {
        &self.sum5
    }
    #[doc = "0x20 - 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum6(&self) -> &SUM6 {
        &self.sum6
    }
    #[doc = "0x24 - 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum7(&self) -> &SUM7 {
        &self.sum7
    }
}
#[doc = "CSR (rw) register accessor: Control and status register  

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control and status register"]
pub mod csr;
#[doc = "WDATA (w) register accessor: Write data register  
 After pulsing START and writing 16 words of data to this register, WDATA_RDY will go low and the SHA-256 core will complete the digest of the current 512-bit block.  

 Software is responsible for ensuring the data is correctly padded and terminated to a whole number of 512-bit blocks.  

 After this, WDATA_RDY will return high, and more data can be written (if any).  

 This register supports word, halfword and byte writes, so that DMA from non-word-aligned buffers can be supported. The total amount of data per block remains the same (16 words, 32 halfwords or 64 bytes) and byte/halfword transfers must not be mixed within a block.  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@wdata`]
module"]
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
#[doc = "Write data register  
 After pulsing START and writing 16 words of data to this register, WDATA_RDY will go low and the SHA-256 core will complete the digest of the current 512-bit block.  

 Software is responsible for ensuring the data is correctly padded and terminated to a whole number of 512-bit blocks.  

 After this, WDATA_RDY will return high, and more data can be written (if any).  

 This register supports word, halfword and byte writes, so that DMA from non-word-aligned buffers can be supported. The total amount of data per block remains the same (16 words, 32 halfwords or 64 bytes) and byte/halfword transfers must not be mixed within a block."]
pub mod wdata;
#[doc = "SUM0 (r) register accessor: 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sum0`]
module"]
pub type SUM0 = crate::Reg<sum0::SUM0_SPEC>;
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
pub mod sum0;
#[doc = "SUM1 (r) register accessor: 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sum1`]
module"]
pub type SUM1 = crate::Reg<sum1::SUM1_SPEC>;
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
pub mod sum1;
#[doc = "SUM2 (r) register accessor: 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sum2`]
module"]
pub type SUM2 = crate::Reg<sum2::SUM2_SPEC>;
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
pub mod sum2;
#[doc = "SUM3 (r) register accessor: 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sum3`]
module"]
pub type SUM3 = crate::Reg<sum3::SUM3_SPEC>;
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
pub mod sum3;
#[doc = "SUM4 (r) register accessor: 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sum4`]
module"]
pub type SUM4 = crate::Reg<sum4::SUM4_SPEC>;
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
pub mod sum4;
#[doc = "SUM5 (r) register accessor: 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sum5`]
module"]
pub type SUM5 = crate::Reg<sum5::SUM5_SPEC>;
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
pub mod sum5;
#[doc = "SUM6 (r) register accessor: 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sum6`]
module"]
pub type SUM6 = crate::Reg<sum6::SUM6_SPEC>;
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
pub mod sum6;
#[doc = "SUM7 (r) register accessor: 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sum7`]
module"]
pub type SUM7 = crate::Reg<sum7::SUM7_SPEC>;
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
pub mod sum7;
