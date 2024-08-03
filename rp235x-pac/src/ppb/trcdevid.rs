#[doc = "Register `TRCDEVID` reader"]
pub type R = crate::R<TRCDEVID_SPEC>;
#[doc = "Register `TRCDEVID` writer"]
pub type W = crate::W<TRCDEVID_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TRCDEVID  

You can [`read`](crate::Reg::read) this register and get [`trcdevid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcdevid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCDEVID_SPEC;
impl crate::RegisterSpec for TRCDEVID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcdevid::R`](R) reader structure"]
impl crate::Readable for TRCDEVID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcdevid::W`](W) writer structure"]
impl crate::Writable for TRCDEVID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCDEVID to value 0"]
impl crate::Resettable for TRCDEVID_SPEC {
    const RESET_VALUE: u32 = 0;
}
