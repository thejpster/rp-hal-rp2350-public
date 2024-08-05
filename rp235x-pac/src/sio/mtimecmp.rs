#[doc = "Register `MTIMECMP` reader"]
pub type R = crate::R<MTIMECMP_SPEC>;
#[doc = "Register `MTIMECMP` writer"]
pub type W = crate::W<MTIMECMP_SPEC>;
#[doc = "Field `MTIMECMP` reader - "]
pub type MTIMECMP_R = crate::FieldReader<u32>;
#[doc = "Field `MTIMECMP` writer - "]
pub type MTIMECMP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mtimecmp(&self) -> MTIMECMP_R {
        MTIMECMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn mtimecmp(&mut self) -> MTIMECMP_W<MTIMECMP_SPEC> {
        MTIMECMP_W::new(self, 0)
    }
}
#[doc = "Low half of RISC-V Machine-mode timer comparator. This register is core-local, i.e., each core gets a copy of this register, with the comparison result routed to its own interrupt line. The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values.  

You can [`read`](crate::Reg::read) this register and get [`mtimecmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMECMP_SPEC;
impl crate::RegisterSpec for MTIMECMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmp::R`](R) reader structure"]
impl crate::Readable for MTIMECMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp::W`](W) writer structure"]
impl crate::Writable for MTIMECMP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTIMECMP to value 0xffff_ffff"]
impl crate::Resettable for MTIMECMP_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
