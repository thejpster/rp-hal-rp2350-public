#[doc = "Register `MHPMEVENT5` reader"]
pub type R = crate::R<MHPMEVENT5_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMEVENT5_SPEC;
impl crate::RegisterSpec for MHPMEVENT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmevent5::R`](R) reader structure"]
impl crate::Readable for MHPMEVENT5_SPEC {}
#[doc = "`reset()` method sets MHPMEVENT5 to value 0"]
impl crate::Resettable for MHPMEVENT5_SPEC {
    const RESET_VALUE: u32 = 0;
}
