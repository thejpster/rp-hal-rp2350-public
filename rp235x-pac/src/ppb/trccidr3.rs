#[doc = "Register `TRCCIDR3` reader"]
pub type R = crate::R<TRCCIDR3_SPEC>;
#[doc = "Field `PRMBL_3` reader - reads as 0b10110001"]
pub type PRMBL_3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - reads as 0b10110001"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TRCCIDR3  

You can [`read`](crate::Reg::read) this register and get [`trccidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCCIDR3_SPEC;
impl crate::RegisterSpec for TRCCIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trccidr3::R`](R) reader structure"]
impl crate::Readable for TRCCIDR3_SPEC {}
#[doc = "`reset()` method sets TRCCIDR3 to value 0xb1"]
impl crate::Resettable for TRCCIDR3_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
