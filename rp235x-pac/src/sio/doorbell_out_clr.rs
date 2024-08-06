#[doc = "Register `DOORBELL_OUT_CLR` reader"]
pub type R = crate::R<DOORBELL_OUT_CLR_SPEC>;
#[doc = "Register `DOORBELL_OUT_CLR` writer"]
pub type W = crate::W<DOORBELL_OUT_CLR_SPEC>;
#[doc = "Field `DOORBELL_OUT_CLR` reader - "]
pub type DOORBELL_OUT_CLR_R = crate::FieldReader;
#[doc = "Field `DOORBELL_OUT_CLR` writer - "]
pub type DOORBELL_OUT_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn doorbell_out_clr(&self) -> DOORBELL_OUT_CLR_R {
        DOORBELL_OUT_CLR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell_out_clr(&mut self) -> DOORBELL_OUT_CLR_W<DOORBELL_OUT_CLR_SPEC> {
        DOORBELL_OUT_CLR_W::new(self, 0)
    }
}
#[doc = "Clear doorbells which have been posted to the opposite core. This register is intended for debugging and initialisation purposes. Writing 1 to a bit in DOORBELL_OUT_CLR clears the corresponding bit in DOORBELL_IN on the opposite core. Clearing all bits will cause that core's doorbell interrupt to deassert. Since the usual order of events is for software to send events using DOORBELL_OUT_SET, and acknowledge incoming events by writing to DOORBELL_IN_CLR, this register should be used with caution to avoid race conditions. Reading returns the status of the doorbells currently asserted on the other core, i.e. is equivalent to that core reading its own DOORBELL_IN status.  

You can [`read`](crate::Reg::read) this register and get [`doorbell_out_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doorbell_out_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOORBELL_OUT_CLR_SPEC;
impl crate::RegisterSpec for DOORBELL_OUT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doorbell_out_clr::R`](R) reader structure"]
impl crate::Readable for DOORBELL_OUT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doorbell_out_clr::W`](W) writer structure"]
impl crate::Writable for DOORBELL_OUT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets DOORBELL_OUT_CLR to value 0"]
impl crate::Resettable for DOORBELL_OUT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
