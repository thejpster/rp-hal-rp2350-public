#[doc = "Register `CYCLEH` reader"]
pub type R = crate::R<CYCLEH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Read-only U-mode alias of mcycleh, accessible when `mcounteren.cy` is set  

You can [`read`](crate::Reg::read) this register and get [`cycleh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CYCLEH_SPEC;
impl crate::RegisterSpec for CYCLEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cycleh::R`](R) reader structure"]
impl crate::Readable for CYCLEH_SPEC {}
#[doc = "`reset()` method sets CYCLEH to value 0"]
impl crate::Resettable for CYCLEH_SPEC {
    const RESET_VALUE: u32 = 0;
}
