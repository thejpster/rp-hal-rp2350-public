#[doc = "Register `MHPMCOUNTER20` reader"]
pub type R = crate::R<MHPMCOUNTER20_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter20::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMCOUNTER20_SPEC;
impl crate::RegisterSpec for MHPMCOUNTER20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmcounter20::R`](R) reader structure"]
impl crate::Readable for MHPMCOUNTER20_SPEC {}
#[doc = "`reset()` method sets MHPMCOUNTER20 to value 0"]
impl crate::Resettable for MHPMCOUNTER20_SPEC {
    const RESET_VALUE: u32 = 0;
}
