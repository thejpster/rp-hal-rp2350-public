#[doc = "Register `MCONFIGPTR` reader"]
pub type R = crate::R<MCONFIGPTR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Pointer to configuration data structure (hardwired to 0)  

You can [`read`](crate::Reg::read) this register and get [`mconfigptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCONFIGPTR_SPEC;
impl crate::RegisterSpec for MCONFIGPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mconfigptr::R`](R) reader structure"]
impl crate::Readable for MCONFIGPTR_SPEC {}
#[doc = "`reset()` method sets MCONFIGPTR to value 0"]
impl crate::Resettable for MCONFIGPTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
