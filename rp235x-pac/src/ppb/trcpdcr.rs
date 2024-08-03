#[doc = "Register `TRCPDCR` reader"]
pub type R = crate::R<TRCPDCR_SPEC>;
#[doc = "Register `TRCPDCR` writer"]
pub type W = crate::W<TRCPDCR_SPEC>;
#[doc = "Field `PU` reader - Powerup request bit:"]
pub type PU_R = crate::BitReader;
#[doc = "Field `PU` writer - Powerup request bit:"]
pub type PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Powerup request bit:"]
    #[inline(always)]
    pub fn pu(&self) -> PU_R {
        PU_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Powerup request bit:"]
    #[inline(always)]
    #[must_use]
    pub fn pu(&mut self) -> PU_W<TRCPDCR_SPEC> {
        PU_W::new(self, 3)
    }
}
#[doc = "Requests the system to provide power to the trace unit  

You can [`read`](crate::Reg::read) this register and get [`trcpdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPDCR_SPEC;
impl crate::RegisterSpec for TRCPDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpdcr::R`](R) reader structure"]
impl crate::Readable for TRCPDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcpdcr::W`](W) writer structure"]
impl crate::Writable for TRCPDCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCPDCR to value 0"]
impl crate::Resettable for TRCPDCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
