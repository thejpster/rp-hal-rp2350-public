#[doc = "Register `TRCIDR13` reader"]
pub type R = crate::R<TRCIDR13_SPEC>;
#[doc = "Field `NUMCONDSPC` reader - reads as `ImpDef"]
pub type NUMCONDSPC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numcondspc(&self) -> NUMCONDSPC_R {
        NUMCONDSPC_R::new(self.bits)
    }
}
#[doc = "TRCIDR13  

You can [`read`](crate::Reg::read) this register and get [`trcidr13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR13_SPEC;
impl crate::RegisterSpec for TRCIDR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr13::R`](R) reader structure"]
impl crate::Readable for TRCIDR13_SPEC {}
#[doc = "`reset()` method sets TRCIDR13 to value 0"]
impl crate::Resettable for TRCIDR13_SPEC {
    const RESET_VALUE: u32 = 0;
}
