#[doc = "Register `MIDELEG` reader"]
pub type R = crate::R<MIDELEG_SPEC>;
#[doc = "Register `MIDELEG` writer"]
pub type W = crate::W<MIDELEG_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine interrupt delegation register. Not implemented, as no S-mode support.  

You can [`read`](crate::Reg::read) this register and get [`mideleg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mideleg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIDELEG_SPEC;
impl crate::RegisterSpec for MIDELEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mideleg::R`](R) reader structure"]
impl crate::Readable for MIDELEG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mideleg::W`](W) writer structure"]
impl crate::Writable for MIDELEG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIDELEG to value 0"]
impl crate::Resettable for MIDELEG_SPEC {
    const RESET_VALUE: u32 = 0;
}
