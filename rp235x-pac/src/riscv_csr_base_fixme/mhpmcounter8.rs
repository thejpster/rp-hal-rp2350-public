#[doc = "Register `MHPMCOUNTER8` reader"]
pub type R = crate::R<MHPMCOUNTER8_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHPMCOUNTER8_SPEC;
impl crate::RegisterSpec for MHPMCOUNTER8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhpmcounter8::R`](R) reader structure"]
impl crate::Readable for MHPMCOUNTER8_SPEC {}
#[doc = "`reset()` method sets MHPMCOUNTER8 to value 0"]
impl crate::Resettable for MHPMCOUNTER8_SPEC {
    const RESET_VALUE: u32 = 0;
}
