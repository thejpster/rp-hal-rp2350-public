#[doc = "Register `SUM2` reader"]
pub type R = crate::R<SUM2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUM2_SPEC;
impl crate::RegisterSpec for SUM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sum2::R`](R) reader structure"]
impl crate::Readable for SUM2_SPEC {}
#[doc = "`reset()` method sets SUM2 to value 0"]
impl crate::Resettable for SUM2_SPEC {
    const RESET_VALUE: u32 = 0;
}
