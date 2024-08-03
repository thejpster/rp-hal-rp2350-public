#[doc = "Register `MHPMCOUNTER25` reader"]
pub type R = crate::R<MHPMCOUNTER25_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter25::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMCOUNTER25_SPEC;
impl crate::RegisterSpec for MHPMCOUNTER25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmcounter25::R`](R) reader structure"]
impl crate::Readable for MHPMCOUNTER25_SPEC {}
#[doc = "`reset()` method sets MHPMCOUNTER25 to value 0"]
impl crate::Resettable for MHPMCOUNTER25_SPEC {
    const RESET_VALUE: u32 = 0;
}
