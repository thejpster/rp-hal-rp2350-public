#[doc = "Register `DPIDR3` reader"]
pub type R = crate::R<DPIDR3_SPEC>;
#[doc = "Register `DPIDR3` writer"]
pub type W = crate::W<DPIDR3_SPEC>;
#[doc = "Field `CMOD` reader - See CoreSight Architecture Specification"]
pub type CMOD_R = crate::FieldReader;
#[doc = "Field `REVAND` reader - See CoreSight Architecture Specification"]
pub type REVAND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPIDR3_SPEC;
impl crate::RegisterSpec for DPIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpidr3::R`](R) reader structure"]
impl crate::Readable for DPIDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpidr3::W`](W) writer structure"]
impl crate::Writable for DPIDR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPIDR3 to value 0"]
impl crate::Resettable for DPIDR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
