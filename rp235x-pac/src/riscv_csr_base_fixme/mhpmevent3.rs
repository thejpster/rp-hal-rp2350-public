#[doc = "Register `MHPMEVENT3` reader"]
pub type R = crate::R<MHPMEVENT3_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMEVENT3_SPEC;
impl crate::RegisterSpec for MHPMEVENT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmevent3::R`](R) reader structure"]
impl crate::Readable for MHPMEVENT3_SPEC {}
#[doc = "`reset()` method sets MHPMEVENT3 to value 0"]
impl crate::Resettable for MHPMEVENT3_SPEC {
    const RESET_VALUE: u32 = 0;
}
