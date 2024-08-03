#[doc = "Register `MHPMCOUNTER4` reader"]
pub type R = crate::R<MHPMCOUNTER4_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMCOUNTER4_SPEC;
impl crate::RegisterSpec for MHPMCOUNTER4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmcounter4::R`](R) reader structure"]
impl crate::Readable for MHPMCOUNTER4_SPEC {}
#[doc = "`reset()` method sets MHPMCOUNTER4 to value 0"]
impl crate::Resettable for MHPMCOUNTER4_SPEC {
    const RESET_VALUE: u32 = 0;
}
