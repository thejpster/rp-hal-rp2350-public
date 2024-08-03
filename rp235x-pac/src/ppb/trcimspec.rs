#[doc = "Register `TRCIMSPEC` reader"]
pub type R = crate::R<TRCIMSPEC_SPEC>;
#[doc = "Field `SUPPORT` reader - Reserved, RES0"]
pub type SUPPORT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Reserved, RES0"]
    #[inline(always)]
    pub fn support(&self) -> SUPPORT_R {
        SUPPORT_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "The TRCIMSPEC shows the presence of any IMPLEMENTATION SPECIFIC features, and enables any features that are provided  

You can [`read`](crate::Reg::read) this register and get [`trcimspec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIMSPEC_SPEC;
impl crate::RegisterSpec for TRCIMSPEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcimspec::R`](R) reader structure"]
impl crate::Readable for TRCIMSPEC_SPEC {}
#[doc = "`reset()` method sets TRCIMSPEC to value 0"]
impl crate::Resettable for TRCIMSPEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
