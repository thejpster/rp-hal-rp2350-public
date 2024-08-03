#[doc = "Register `MHPMEVENT28` reader"]
pub type R = crate::R<MHPMEVENT28_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMEVENT28_SPEC;
impl crate::RegisterSpec for MHPMEVENT28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmevent28::R`](R) reader structure"]
impl crate::Readable for MHPMEVENT28_SPEC {}
#[doc = "`reset()` method sets MHPMEVENT28 to value 0"]
impl crate::Resettable for MHPMEVENT28_SPEC {
    const RESET_VALUE: u32 = 0;
}
