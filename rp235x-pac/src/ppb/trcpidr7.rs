#[doc = "Register `TRCPIDR7` reader"]
pub type R = crate::R<TRCPIDR7_SPEC>;
#[doc = "Register `TRCPIDR7` writer"]
pub type W = crate::W<TRCPIDR7_SPEC>;
#[doc = "Field `TRCPIDR7` reader - "]
pub type TRCPIDR7_R = crate::FieldReader<u32>;
#[doc = "Field `TRCPIDR7` writer - "]
pub type TRCPIDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn trcpidr7(&self) -> TRCPIDR7_R {
        TRCPIDR7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn trcpidr7(&mut self) -> TRCPIDR7_W<TRCPIDR7_SPEC> {
        TRCPIDR7_W::new(self, 0)
    }
}
#[doc = "TRCPIDR7  

You can [`read`](crate::Reg::read) this register and get [`trcpidr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPIDR7_SPEC;
impl crate::RegisterSpec for TRCPIDR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpidr7::R`](R) reader structure"]
impl crate::Readable for TRCPIDR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcpidr7::W`](W) writer structure"]
impl crate::Writable for TRCPIDR7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCPIDR7 to value 0"]
impl crate::Resettable for TRCPIDR7_SPEC {
    const RESET_VALUE: u32 = 0;
}
