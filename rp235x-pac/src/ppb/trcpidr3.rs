#[doc = "Register `TRCPIDR3` reader"]
pub type R = crate::R<TRCPIDR3_SPEC>;
#[doc = "Register `TRCPIDR3` writer"]
pub type W = crate::W<TRCPIDR3_SPEC>;
#[doc = "Field `CMOD` reader - reads as `ImpDef"]
pub type CMOD_R = crate::FieldReader;
#[doc = "Field `REVAND` reader - reads as `ImpDef"]
pub type REVAND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - reads as `ImpDef"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - reads as `ImpDef"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "TRCPIDR3  

You can [`read`](crate::Reg::read) this register and get [`trcpidr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPIDR3_SPEC;
impl crate::RegisterSpec for TRCPIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpidr3::R`](R) reader structure"]
impl crate::Readable for TRCPIDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcpidr3::W`](W) writer structure"]
impl crate::Writable for TRCPIDR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCPIDR3 to value 0"]
impl crate::Resettable for TRCPIDR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
