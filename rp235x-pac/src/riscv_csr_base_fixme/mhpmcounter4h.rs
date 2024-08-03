#[doc = "Register `MHPMCOUNTER4H` reader"]
pub type R = crate::R<MHPMCOUNTER4H_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter4h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMCOUNTER4H_SPEC;
impl crate::RegisterSpec for MHPMCOUNTER4H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmcounter4h::R`](R) reader structure"]
impl crate::Readable for MHPMCOUNTER4H_SPEC {}
#[doc = "`reset()` method sets MHPMCOUNTER4H to value 0"]
impl crate::Resettable for MHPMCOUNTER4H_SPEC {
    const RESET_VALUE: u32 = 0;
}