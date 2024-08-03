#[doc = "Register `TRCIDR11` reader"]
pub type R = crate::R<TRCIDR11_SPEC>;
#[doc = "Field `NUMP1SPC` reader - reads as `ImpDef"]
pub type NUMP1SPC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn nump1spc(&self) -> NUMP1SPC_R {
        NUMP1SPC_R::new(self.bits)
    }
}
#[doc = "TRCIDR11  

You can [`read`](crate::Reg::read) this register and get [`trcidr11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR11_SPEC;
impl crate::RegisterSpec for TRCIDR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr11::R`](R) reader structure"]
impl crate::Readable for TRCIDR11_SPEC {}
#[doc = "`reset()` method sets TRCIDR11 to value 0"]
impl crate::Resettable for TRCIDR11_SPEC {
    const RESET_VALUE: u32 = 0;
}
