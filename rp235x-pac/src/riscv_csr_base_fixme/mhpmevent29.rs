#[doc = "Register `MHPMEVENT29` reader"]
pub type R = crate::R<MHPMEVENT29_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent29::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMEVENT29_SPEC;
impl crate::RegisterSpec for MHPMEVENT29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmevent29::R`](R) reader structure"]
impl crate::Readable for MHPMEVENT29_SPEC {}
#[doc = "`reset()` method sets MHPMEVENT29 to value 0"]
impl crate::Resettable for MHPMEVENT29_SPEC {
    const RESET_VALUE: u32 = 0;
}
