#[doc = "Register `MEDELEG` reader"]
pub type R = crate::R<MEDELEG_SPEC>;
#[doc = "Register `MEDELEG` writer"]
pub type W = crate::W<MEDELEG_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine exception delegation register. Not implemented, as no S-mode support.  

You can [`read`](crate::Reg::read) this register and get [`medeleg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`medeleg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEDELEG_SPEC;
impl crate::RegisterSpec for MEDELEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`medeleg::R`](R) reader structure"]
impl crate::Readable for MEDELEG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`medeleg::W`](W) writer structure"]
impl crate::Writable for MEDELEG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEDELEG to value 0"]
impl crate::Resettable for MEDELEG_SPEC {
    const RESET_VALUE: u32 = 0;
}
