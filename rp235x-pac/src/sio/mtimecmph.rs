#[doc = "Register `MTIMECMPH` reader"]
pub type R = crate::R<MTIMECMPH_SPEC>;
#[doc = "Register `MTIMECMPH` writer"]
pub type W = crate::W<MTIMECMPH_SPEC>;
#[doc = "Field `MTIMECMPH` reader - "]
pub type MTIMECMPH_R = crate::FieldReader<u32>;
#[doc = "Field `MTIMECMPH` writer - "]
pub type MTIMECMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mtimecmph(&self) -> MTIMECMPH_R {
        MTIMECMPH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn mtimecmph(&mut self) -> MTIMECMPH_W<MTIMECMPH_SPEC> {
        MTIMECMPH_W::new(self, 0)
    }
}
#[doc = "High half of RISC-V Machine-mode timer comparator. This register is core-local. The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values.  

You can [`read`](crate::Reg::read) this register and get [`mtimecmph::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmph::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMECMPH_SPEC;
impl crate::RegisterSpec for MTIMECMPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmph::R`](R) reader structure"]
impl crate::Readable for MTIMECMPH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimecmph::W`](W) writer structure"]
impl crate::Writable for MTIMECMPH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTIMECMPH to value 0xffff_ffff"]
impl crate::Resettable for MTIMECMPH_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
