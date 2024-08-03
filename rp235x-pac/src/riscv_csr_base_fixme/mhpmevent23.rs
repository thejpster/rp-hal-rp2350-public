#[doc = "Register `MHPMEVENT23` reader"]
pub type R = crate::R<MHPMEVENT23_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent23::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMEVENT23_SPEC;
impl crate::RegisterSpec for MHPMEVENT23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmevent23::R`](R) reader structure"]
impl crate::Readable for MHPMEVENT23_SPEC {}
#[doc = "`reset()` method sets MHPMEVENT23 to value 0"]
impl crate::Resettable for MHPMEVENT23_SPEC {
    const RESET_VALUE: u32 = 0;
}
