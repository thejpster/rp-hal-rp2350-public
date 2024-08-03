#[doc = "Register `CYCLE` reader"]
pub type R = crate::R<CYCLE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Read-only U-mode alias of mcycle, accessible when `mcounteren.cy` is set  

You can [`read`](crate::Reg::read) this register and get [`cycle::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CYCLE_SPEC;
impl crate::RegisterSpec for CYCLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cycle::R`](R) reader structure"]
impl crate::Readable for CYCLE_SPEC {}
#[doc = "`reset()` method sets CYCLE to value 0"]
impl crate::Resettable for CYCLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
