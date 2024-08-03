#[doc = "Register `FP_PIDR3` reader"]
pub type R = crate::R<FP_PIDR3_SPEC>;
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
#[doc = "Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_PIDR3_SPEC;
impl crate::RegisterSpec for FP_PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_pidr3::R`](R) reader structure"]
impl crate::Readable for FP_PIDR3_SPEC {}
#[doc = "`reset()` method sets FP_PIDR3 to value 0"]
impl crate::Resettable for FP_PIDR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
