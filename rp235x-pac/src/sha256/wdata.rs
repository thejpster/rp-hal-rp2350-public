#[doc = "Register `WDATA` reader"]
pub type R = crate::R<WDATA_SPEC>;
#[doc = "Register `WDATA` writer"]
pub type W = crate::W<WDATA_SPEC>;
#[doc = "Field `WDATA` writer - After pulsing START and writing 16 words of data to this register, WDATA_RDY will go low and the SHA-256 core will complete the digest of the current 512-bit block. Software is responsible for ensuring the data is correctly padded and terminated to a whole number of 512-bit blocks. After this, WDATA_RDY will return high, and more data can be written (if any). This register supports word, halfword and byte writes, so that DMA from non-word-aligned buffers can be supported. The total amount of data per block remains the same (16 words, 32 halfwords or 64 bytes) and byte/halfword transfers must not be mixed within a block."]
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - After pulsing START and writing 16 words of data to this register, WDATA_RDY will go low and the SHA-256 core will complete the digest of the current 512-bit block. Software is responsible for ensuring the data is correctly padded and terminated to a whole number of 512-bit blocks. After this, WDATA_RDY will return high, and more data can be written (if any). This register supports word, halfword and byte writes, so that DMA from non-word-aligned buffers can be supported. The total amount of data per block remains the same (16 words, 32 halfwords or 64 bytes) and byte/halfword transfers must not be mixed within a block."]
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<WDATA_SPEC> {
        WDATA_W::new(self, 0)
    }
}
#[doc = "Write data register  

You can [`read`](crate::Reg::read) this register and get [`wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDATA_SPEC;
impl crate::RegisterSpec for WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdata::R`](R) reader structure"]
impl crate::Readable for WDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdata::W`](W) writer structure"]
impl crate::Writable for WDATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
