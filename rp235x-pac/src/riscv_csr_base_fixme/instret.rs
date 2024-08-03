#[doc = "Register `INSTRET` reader"]
pub type R = crate::R<INSTRET_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Read-only U-mode alias of minstret, accessible when `mcounteren.ir` is set  

You can [`read`](crate::Reg::read) this register and get [`instret::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSTRET_SPEC;
impl crate::RegisterSpec for INSTRET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instret::R`](R) reader structure"]
impl crate::Readable for INSTRET_SPEC {}
#[doc = "`reset()` method sets INSTRET to value 0"]
impl crate::Resettable for INSTRET_SPEC {
    const RESET_VALUE: u32 = 0;
}
