#[doc = "Register `MHARTID` reader"]
pub type R = crate::R<MHARTID_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Hardware thread ID  
 On RP2350, core 0 has a hart ID of 0, and core 1 has a hart ID of 1.  

You can [`read`](crate::Reg::read) this register and get [`mhartid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MHARTID_SPEC;
impl crate::RegisterSpec for MHARTID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhartid::R`](R) reader structure"]
impl crate::Readable for MHARTID_SPEC {}
#[doc = "`reset()` method sets MHARTID to value 0"]
impl crate::Resettable for MHARTID_SPEC {
    const RESET_VALUE: u32 = 0;
}
