#[doc = "Register `MHPMCOUNTER30` reader"]
pub type R = crate::R<MHPMCOUNTER30_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter30::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMCOUNTER30_SPEC;
impl crate::RegisterSpec for MHPMCOUNTER30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmcounter30::R`](R) reader structure"]
impl crate::Readable for MHPMCOUNTER30_SPEC {}
#[doc = "`reset()` method sets MHPMCOUNTER30 to value 0"]
impl crate::Resettable for MHPMCOUNTER30_SPEC {
    const RESET_VALUE: u32 = 0;
}
