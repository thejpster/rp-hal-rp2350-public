#[doc = "Register `MHPMEVENT6` reader"]
pub type R = crate::R<MHPMEVENT6_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMEVENT6_SPEC;
impl crate::RegisterSpec for MHPMEVENT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmevent6::R`](R) reader structure"]
impl crate::Readable for MHPMEVENT6_SPEC {}
#[doc = "`reset()` method sets MHPMEVENT6 to value 0"]
impl crate::Resettable for MHPMEVENT6_SPEC {
    const RESET_VALUE: u32 = 0;
}
