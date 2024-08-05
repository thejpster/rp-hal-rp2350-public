#[doc = "Register `TRCIMSPEC` reader"]
pub type R = crate::R<TRCIMSPEC_SPEC>;
#[doc = "Register `TRCIMSPEC` writer"]
pub type W = crate::W<TRCIMSPEC_SPEC>;
#[doc = "Field `SUPPORT` reader - Reserved, RES0"]
pub type SUPPORT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Reserved, RES0"]
    #[inline(always)]
    pub fn support(&self) -> SUPPORT_R {
        SUPPORT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {}
#[doc = "The TRCIMSPEC shows the presence of any IMPLEMENTATION SPECIFIC features, and enables any features that are provided  

You can [`read`](crate::Reg::read) this register and get [`trcimspec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcimspec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIMSPEC_SPEC;
impl crate::RegisterSpec for TRCIMSPEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcimspec::R`](R) reader structure"]
impl crate::Readable for TRCIMSPEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcimspec::W`](W) writer structure"]
impl crate::Writable for TRCIMSPEC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIMSPEC to value 0"]
impl crate::Resettable for TRCIMSPEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
