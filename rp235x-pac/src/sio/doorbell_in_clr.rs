#[doc = "Register `DOORBELL_IN_CLR` reader"]
pub type R = crate::R<DOORBELL_IN_CLR_SPEC>;
#[doc = "Register `DOORBELL_IN_CLR` writer"]
pub type W = crate::W<DOORBELL_IN_CLR_SPEC>;
#[doc = "Field `DOORBELL_IN_CLR` reader - "]
pub type DOORBELL_IN_CLR_R = crate::FieldReader;
#[doc = "Field `DOORBELL_IN_CLR` writer - "]
pub type DOORBELL_IN_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn doorbell_in_clr(&self) -> DOORBELL_IN_CLR_R {
        DOORBELL_IN_CLR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell_in_clr(&mut self) -> DOORBELL_IN_CLR_W<DOORBELL_IN_CLR_SPEC> {
        DOORBELL_IN_CLR_W::new(self, 0)
    }
}
#[doc = "Check and acknowledge doorbells posted to this core. This core's doorbell interrupt is asserted when any bit in this register is 1. Write 1 to each bit to clear that bit. The doorbell interrupt deasserts once all bits are cleared. Read to get status of doorbells currently asserted on this core.  

You can [`read`](crate::Reg::read) this register and get [`doorbell_in_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doorbell_in_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOORBELL_IN_CLR_SPEC;
impl crate::RegisterSpec for DOORBELL_IN_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doorbell_in_clr::R`](R) reader structure"]
impl crate::Readable for DOORBELL_IN_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doorbell_in_clr::W`](W) writer structure"]
impl crate::Writable for DOORBELL_IN_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets DOORBELL_IN_CLR to value 0"]
impl crate::Resettable for DOORBELL_IN_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
