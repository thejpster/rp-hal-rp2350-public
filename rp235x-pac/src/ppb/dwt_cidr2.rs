#[doc = "Register `DWT_CIDR2` reader"]
pub type R = crate::R<DWT_CIDR2_SPEC>;
#[doc = "Field `PRMBL_2` reader - See CoreSight Architecture Specification"]
pub type PRMBL_2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_cidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_CIDR2_SPEC;
impl crate::RegisterSpec for DWT_CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_cidr2::R`](R) reader structure"]
impl crate::Readable for DWT_CIDR2_SPEC {}
#[doc = "`reset()` method sets DWT_CIDR2 to value 0x05"]
impl crate::Resettable for DWT_CIDR2_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
