#[doc = "Register `DPIDR5` reader"]
pub type R = crate::R<DPIDR5_SPEC>;
#[doc = "Register `DPIDR5` writer"]
pub type W = crate::W<DPIDR5_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPIDR5_SPEC;
impl crate::RegisterSpec for DPIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpidr5::R`](R) reader structure"]
impl crate::Readable for DPIDR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpidr5::W`](W) writer structure"]
impl crate::Writable for DPIDR5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPIDR5 to value 0"]
impl crate::Resettable for DPIDR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
