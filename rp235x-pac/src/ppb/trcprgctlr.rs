#[doc = "Register `TRCPRGCTLR` reader"]
pub type R = crate::R<TRCPRGCTLR_SPEC>;
#[doc = "Register `TRCPRGCTLR` writer"]
pub type W = crate::W<TRCPRGCTLR_SPEC>;
#[doc = "Field `EN` reader - Trace Unit Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Trace Unit Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Trace Unit Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trace Unit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<TRCPRGCTLR_SPEC> {
        EN_W::new(self, 0)
    }
}
#[doc = "Programming Control Register  

You can [`read`](crate::Reg::read) this register and get [`trcprgctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcprgctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPRGCTLR_SPEC;
impl crate::RegisterSpec for TRCPRGCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcprgctlr::R`](R) reader structure"]
impl crate::Readable for TRCPRGCTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcprgctlr::W`](W) writer structure"]
impl crate::Writable for TRCPRGCTLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCPRGCTLR to value 0"]
impl crate::Resettable for TRCPRGCTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
