#[doc = "Register `MARCHID` reader"]
pub type R = crate::R<MARCHID_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Architecture ID (Hazard3)  

You can [`read`](crate::Reg::read) this register and get [`marchid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MARCHID_SPEC;
impl crate::RegisterSpec for MARCHID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`marchid::R`](R) reader structure"]
impl crate::Readable for MARCHID_SPEC {}
#[doc = "`reset()` method sets MARCHID to value 0x1b"]
impl crate::Resettable for MARCHID_SPEC {
    const RESET_VALUE: u32 = 0x1b;
}
