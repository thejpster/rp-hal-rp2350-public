#[doc = "Register `TRCPIDR6` reader"]
pub type R = crate::R<TRCPIDR6_SPEC>;
#[doc = "Register `TRCPIDR6` writer"]
pub type W = crate::W<TRCPIDR6_SPEC>;
#[doc = "Field `TRCPIDR6` reader - "]
pub type TRCPIDR6_R = crate::FieldReader<u32>;
#[doc = "Field `TRCPIDR6` writer - "]
pub type TRCPIDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn trcpidr6(&self) -> TRCPIDR6_R {
        TRCPIDR6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn trcpidr6(&mut self) -> TRCPIDR6_W<TRCPIDR6_SPEC> {
        TRCPIDR6_W::new(self, 0)
    }
}
#[doc = "TRCPIDR6  

You can [`read`](crate::Reg::read) this register and get [`trcpidr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPIDR6_SPEC;
impl crate::RegisterSpec for TRCPIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpidr6::R`](R) reader structure"]
impl crate::Readable for TRCPIDR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcpidr6::W`](W) writer structure"]
impl crate::Writable for TRCPIDR6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCPIDR6 to value 0"]
impl crate::Resettable for TRCPIDR6_SPEC {
    const RESET_VALUE: u32 = 0;
}
