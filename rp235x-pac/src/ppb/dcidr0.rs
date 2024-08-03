#[doc = "Register `DCIDR0` reader"]
pub type R = crate::R<DCIDR0_SPEC>;
#[doc = "Field `PRMBL_0` reader - See CoreSight Architecture Specification"]
pub type PRMBL_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dcidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCIDR0_SPEC;
impl crate::RegisterSpec for DCIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcidr0::R`](R) reader structure"]
impl crate::Readable for DCIDR0_SPEC {}
#[doc = "`reset()` method sets DCIDR0 to value 0x0d"]
impl crate::Resettable for DCIDR0_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
