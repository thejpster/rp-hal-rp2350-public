#[doc = "Register `MEIEA` reader"]
pub type R = crate::R<MEIEA_SPEC>;
#[doc = "Register `MEIEA` writer"]
pub type W = crate::W<MEIEA_SPEC>;
#[doc = "Field `INDEX` writer - Write-only self-clearing field (no value is stored) used to control which window of the array appears in `window`."]
pub type INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WINDOW` reader - 16-bit read/write window into the external interrupt enable array"]
pub type WINDOW_R = crate::FieldReader<u16>;
#[doc = "Field `WINDOW` writer - 16-bit read/write window into the external interrupt enable array"]
pub type WINDOW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - 16-bit read/write window into the external interrupt enable array"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Write-only self-clearing field (no value is stored) used to control which window of the array appears in `window`."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<MEIEA_SPEC> {
        INDEX_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 16-bit read/write window into the external interrupt enable array"]
    #[inline(always)]
    #[must_use]
    pub fn window(&mut self) -> WINDOW_W<MEIEA_SPEC> {
        WINDOW_W::new(self, 16)
    }
}
#[doc = "External interrupt enable array.  

 The array contains a read-write bit for each external interrupt request: a `1` bit indicates that interrupt is currently enabled. At reset, all external interrupts are disabled.  

 If enabled, an external interrupt can cause assertion of the standard RISC-V machine external interrupt pending flag (`mip.meip`), and therefore cause the processor to enter the external interrupt vector. See `meipa`.  

 There are up to 512 external interrupts. The upper half of this register contains a 16-bit window into the full 512-bit vector. The window is indexed by the 5 LSBs of the write data.  

You can [`read`](crate::Reg::read) this register and get [`meiea::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meiea::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEIEA_SPEC;
impl crate::RegisterSpec for MEIEA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meiea::R`](R) reader structure"]
impl crate::Readable for MEIEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`meiea::W`](W) writer structure"]
impl crate::Writable for MEIEA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEIEA to value 0"]
impl crate::Resettable for MEIEA_SPEC {
    const RESET_VALUE: u32 = 0;
}
