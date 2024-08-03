#[doc = "Register `MSTATUSH` reader"]
pub type R = crate::R<MSTATUSH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "High half of mstatus, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mstatush::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSTATUSH_SPEC;
impl crate::RegisterSpec for MSTATUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstatush::R`](R) reader structure"]
impl crate::Readable for MSTATUSH_SPEC {}
#[doc = "`reset()` method sets MSTATUSH to value 0"]
impl crate::Resettable for MSTATUSH_SPEC {
    const RESET_VALUE: u32 = 0;
}
