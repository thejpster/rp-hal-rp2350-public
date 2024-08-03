#[doc = "Register `TRCCIDR0` reader"]
pub type R = crate::R<TRCCIDR0_SPEC>;
#[doc = "Field `PRMBL_0` reader - reads as 0b00001101"]
pub type PRMBL_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - reads as 0b00001101"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TRCCIDR0  

You can [`read`](crate::Reg::read) this register and get [`trccidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCCIDR0_SPEC;
impl crate::RegisterSpec for TRCCIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trccidr0::R`](R) reader structure"]
impl crate::Readable for TRCCIDR0_SPEC {}
#[doc = "`reset()` method sets TRCCIDR0 to value 0x0d"]
impl crate::Resettable for TRCCIDR0_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
