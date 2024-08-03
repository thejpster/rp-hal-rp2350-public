#[doc = "Register `FP_CIDR1` reader"]
pub type R = crate::R<FP_CIDR1_SPEC>;
#[doc = "Field `PRMBL_1` reader - See CoreSight Architecture Specification"]
pub type PRMBL_1_R = crate::FieldReader;
#[doc = "Field `CLASS` reader - See CoreSight Architecture Specification"]
pub type CLASS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_cidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_CIDR1_SPEC;
impl crate::RegisterSpec for FP_CIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_cidr1::R`](R) reader structure"]
impl crate::Readable for FP_CIDR1_SPEC {}
#[doc = "`reset()` method sets FP_CIDR1 to value 0x90"]
impl crate::Resettable for FP_CIDR1_SPEC {
    const RESET_VALUE: u32 = 0x90;
}
