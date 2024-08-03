#[doc = "Register `SUM0` reader"]
pub type R = crate::R<SUM0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUM0_SPEC;
impl crate::RegisterSpec for SUM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sum0::R`](R) reader structure"]
impl crate::Readable for SUM0_SPEC {}
#[doc = "`reset()` method sets SUM0 to value 0"]
impl crate::Resettable for SUM0_SPEC {
    const RESET_VALUE: u32 = 0;
}
