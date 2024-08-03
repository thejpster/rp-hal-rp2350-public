#[doc = "Register `DWT_CIDR3` reader"]
pub type R = crate::R<DWT_CIDR3_SPEC>;
#[doc = "Field `PRMBL_3` reader - See CoreSight Architecture Specification"]
pub type PRMBL_3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_cidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_CIDR3_SPEC;
impl crate::RegisterSpec for DWT_CIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_cidr3::R`](R) reader structure"]
impl crate::Readable for DWT_CIDR3_SPEC {}
#[doc = "`reset()` method sets DWT_CIDR3 to value 0xb1"]
impl crate::Resettable for DWT_CIDR3_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
