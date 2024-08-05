#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSR_SPEC>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSR_SPEC>;
#[doc = "Field `START` writer - Write 1 to prepare the SHA-256 core for a new checksum. The SUMx registers are initialised to the proper values (fractional bits of square roots of first 8 primes) and internal counters are cleared. This immediately forces WDATA_RDY and SUM_VLD high. START must be written before initiating a DMA transfer to the SHA-256 core, because the core will always request 16 transfers at a time (1 512-bit block). Additionally, the DMA channel should be configured for a multiple of 16 32-bit transfers."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDATA_RDY` reader - If 1, the SHA-256 core is ready to accept more data through the WDATA register. After writing 16 words, this flag will go low for 57 cycles whilst the core completes its digest."]
pub type WDATA_RDY_R = crate::BitReader;
#[doc = "Field `SUM_VLD` reader - If 1, the SHA-256 checksum presented in registers SUM0 through SUM7 is currently valid. Goes low when WDATA is first written, then returns high once 16 words have been written and the digest of the current 512-bit block has subsequently completed."]
pub type SUM_VLD_R = crate::BitReader;
#[doc = "Field `ERR_WDATA_NOT_RDY` reader - Set when a write occurs whilst the SHA-256 core is not ready for data (WDATA_RDY is low). Write one to clear."]
pub type ERR_WDATA_NOT_RDY_R = crate::BitReader;
#[doc = "Field `ERR_WDATA_NOT_RDY` writer - Set when a write occurs whilst the SHA-256 core is not ready for data (WDATA_RDY is low). Write one to clear."]
pub type ERR_WDATA_NOT_RDY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered. The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block.  

Value on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA_SIZE_A {
    #[doc = "0: `0`"]
    _8BIT = 0,
    #[doc = "1: `1`"]
    _16BIT = 1,
    #[doc = "2: `10`"]
    _32BIT = 2,
}
impl From<DMA_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA_SIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for DMA_SIZE_A {}
#[doc = "Field `DMA_SIZE` reader - Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered. The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block."]
pub type DMA_SIZE_R = crate::FieldReader<DMA_SIZE_A>;
impl DMA_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMA_SIZE_A> {
        match self.bits {
            0 => Some(DMA_SIZE_A::_8BIT),
            1 => Some(DMA_SIZE_A::_16BIT),
            2 => Some(DMA_SIZE_A::_32BIT),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == DMA_SIZE_A::_8BIT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == DMA_SIZE_A::_16BIT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == DMA_SIZE_A::_32BIT
    }
}
#[doc = "Field `DMA_SIZE` writer - Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered. The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block."]
pub type DMA_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DMA_SIZE_A>;
impl<'a, REG> DMA_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_SIZE_A::_8BIT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_SIZE_A::_16BIT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_SIZE_A::_32BIT)
    }
}
#[doc = "Field `BSWAP` reader - Enable byte swapping of 32-bit values at the point they are committed to the SHA message scheduler. This block's bus interface assembles byte/halfword data into message words in little-endian order, so that DMAing the same buffer with different transfer sizes always gives the same result on a little-endian system like RP2350. However, when marshalling bytes into blocks, SHA expects that the first byte is the *most significant* in each message word. To resolve this, once the bus interface has accumulated 32 bits of data (either a word write, two halfword writes in little-endian order, or four byte writes in little-endian order) the final value can be byte-swapped before passing to the actual SHA core. This feature is enabled by default because using the SHA core to checksum byte buffers is expected to be more common than having preformatted SHA message words lying around."]
pub type BSWAP_R = crate::BitReader;
#[doc = "Field `BSWAP` writer - Enable byte swapping of 32-bit values at the point they are committed to the SHA message scheduler. This block's bus interface assembles byte/halfword data into message words in little-endian order, so that DMAing the same buffer with different transfer sizes always gives the same result on a little-endian system like RP2350. However, when marshalling bytes into blocks, SHA expects that the first byte is the *most significant* in each message word. To resolve this, once the bus interface has accumulated 32 bits of data (either a word write, two halfword writes in little-endian order, or four byte writes in little-endian order) the final value can be byte-swapped before passing to the actual SHA core. This feature is enabled by default because using the SHA core to checksum byte buffers is expected to be more common than having preformatted SHA message words lying around."]
pub type BSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - If 1, the SHA-256 core is ready to accept more data through the WDATA register. After writing 16 words, this flag will go low for 57 cycles whilst the core completes its digest."]
    #[inline(always)]
    pub fn wdata_rdy(&self) -> WDATA_RDY_R {
        WDATA_RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If 1, the SHA-256 checksum presented in registers SUM0 through SUM7 is currently valid. Goes low when WDATA is first written, then returns high once 16 words have been written and the digest of the current 512-bit block has subsequently completed."]
    #[inline(always)]
    pub fn sum_vld(&self) -> SUM_VLD_R {
        SUM_VLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Set when a write occurs whilst the SHA-256 core is not ready for data (WDATA_RDY is low). Write one to clear."]
    #[inline(always)]
    pub fn err_wdata_not_rdy(&self) -> ERR_WDATA_NOT_RDY_R {
        ERR_WDATA_NOT_RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered. The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block."]
    #[inline(always)]
    pub fn dma_size(&self) -> DMA_SIZE_R {
        DMA_SIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Enable byte swapping of 32-bit values at the point they are committed to the SHA message scheduler. This block's bus interface assembles byte/halfword data into message words in little-endian order, so that DMAing the same buffer with different transfer sizes always gives the same result on a little-endian system like RP2350. However, when marshalling bytes into blocks, SHA expects that the first byte is the *most significant* in each message word. To resolve this, once the bus interface has accumulated 32 bits of data (either a word write, two halfword writes in little-endian order, or four byte writes in little-endian order) the final value can be byte-swapped before passing to the actual SHA core. This feature is enabled by default because using the SHA core to checksum byte buffers is expected to be more common than having preformatted SHA message words lying around."]
    #[inline(always)]
    pub fn bswap(&self) -> BSWAP_R {
        BSWAP_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to prepare the SHA-256 core for a new checksum. The SUMx registers are initialised to the proper values (fractional bits of square roots of first 8 primes) and internal counters are cleared. This immediately forces WDATA_RDY and SUM_VLD high. START must be written before initiating a DMA transfer to the SHA-256 core, because the core will always request 16 transfers at a time (1 512-bit block). Additionally, the DMA channel should be configured for a multiple of 16 32-bit transfers."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CSR_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 4 - Set when a write occurs whilst the SHA-256 core is not ready for data (WDATA_RDY is low). Write one to clear."]
    #[inline(always)]
    #[must_use]
    pub fn err_wdata_not_rdy(&mut self) -> ERR_WDATA_NOT_RDY_W<CSR_SPEC> {
        ERR_WDATA_NOT_RDY_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered. The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block."]
    #[inline(always)]
    #[must_use]
    pub fn dma_size(&mut self) -> DMA_SIZE_W<CSR_SPEC> {
        DMA_SIZE_W::new(self, 8)
    }
    #[doc = "Bit 12 - Enable byte swapping of 32-bit values at the point they are committed to the SHA message scheduler. This block's bus interface assembles byte/halfword data into message words in little-endian order, so that DMAing the same buffer with different transfer sizes always gives the same result on a little-endian system like RP2350. However, when marshalling bytes into blocks, SHA expects that the first byte is the *most significant* in each message word. To resolve this, once the bus interface has accumulated 32 bits of data (either a word write, two halfword writes in little-endian order, or four byte writes in little-endian order) the final value can be byte-swapped before passing to the actual SHA core. This feature is enabled by default because using the SHA core to checksum byte buffers is expected to be more common than having preformatted SHA message words lying around."]
    #[inline(always)]
    #[must_use]
    pub fn bswap(&mut self) -> BSWAP_W<CSR_SPEC> {
        BSWAP_W::new(self, 12)
    }
}
#[doc = "Control and status register  

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x10;
}
#[doc = "`reset()` method sets CSR to value 0x1206"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: u32 = 0x1206;
}
