#[doc = "Register `NEXTDM` reader"]
pub type R = crate::R<NEXTDM_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "If there is more than one DM accessible on this DMI, this register contains the base address of thenext one in the chain, or 0 if this is the last one in the chain.  

You can [`read`](crate::Reg::read) this register and get [`nextdm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NEXTDM_SPEC;
impl crate::RegisterSpec for NEXTDM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nextdm::R`](R) reader structure"]
impl crate::Readable for NEXTDM_SPEC {}
#[doc = "`reset()` method sets NEXTDM to value 0"]
impl crate::Resettable for NEXTDM_SPEC {
    const RESET_VALUE: u32 = 0;
}
