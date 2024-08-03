#[doc = "Register `ITM_PIDR3` reader"]
pub type R = crate::R<ITM_PIDR3_SPEC>;
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
#[doc = "Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_PIDR3_SPEC;
impl crate::RegisterSpec for ITM_PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_pidr3::R`](R) reader structure"]
impl crate::Readable for ITM_PIDR3_SPEC {}
#[doc = "`reset()` method sets ITM_PIDR3 to value 0"]
impl crate::Resettable for ITM_PIDR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
