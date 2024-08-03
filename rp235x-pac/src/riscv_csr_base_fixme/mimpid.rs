#[doc = "Register `MIMPID` reader"]
pub type R = crate::R<MIMPID_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Implementation ID  

You can [`read`](crate::Reg::read) this register and get [`mimpid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIMPID_SPEC;
impl crate::RegisterSpec for MIMPID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mimpid::R`](R) reader structure"]
impl crate::Readable for MIMPID_SPEC {}
#[doc = "`reset()` method sets MIMPID to value 0"]
impl crate::Resettable for MIMPID_SPEC {
    const RESET_VALUE: u32 = 0;
}
