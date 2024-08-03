#[doc = "Register `MHPMCOUNTER28` reader"]
pub type R = crate::R<MHPMCOUNTER28_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMCOUNTER28_SPEC;
impl crate::RegisterSpec for MHPMCOUNTER28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmcounter28::R`](R) reader structure"]
impl crate::Readable for MHPMCOUNTER28_SPEC {}
#[doc = "`reset()` method sets MHPMCOUNTER28 to value 0"]
impl crate::Resettable for MHPMCOUNTER28_SPEC {
    const RESET_VALUE: u32 = 0;
}
